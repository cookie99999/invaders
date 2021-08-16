#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include "8080.h"
#include "eval.h"
#include "disas.h"

static void unimplemented_instr(system_state* state) {
  printf("<ERROR> unimplemented instruction\n");
  //state->pc--;
  disas_opcode(state->memory, state->pc);
  printf("\n");
  exit(1);
}

static uint8_t parity8(uint8_t byte) {
  byte ^= (byte >> 4);
  byte ^= (byte >> 2);
  byte ^= (byte >> 1);

  return !(byte & 0x01); //8080 uses 1 for even
}

static void mov_8_reg_reg(uint8_t* dest, uint8_t* src) {
  *dest = *src;
}

static void mov_8_imm_reg(uint8_t* dest, uint8_t value, system_state* state) {
  *dest = value;
  state->pc++; //2 byte instruction
}

static void mov_8_imm_mem(system_state* state, uint8_t value) {
  mem_write(state, value, (uint16_t) (state->h << 8) | state->l);
  state->pc++; //2 byte instruction
}

static void ret_jmp_call_rst(uint8_t* opcode, uint8_t low, uint8_t high, system_state* state) {
  uint8_t instr_bits = (*opcode & 0x06) >> 1;
  uint8_t cond_bits = (*opcode & 0x38) >> 3;
  uint8_t extra_bit = *opcode & 1;
  uint16_t ret = state->pc + 2;
  uint16_t rst_ret = state->pc; //pc is already incremented before evaluation

  uint8_t conditions[] = {
			  !state->f.z, state->f.z,
			  !state->f.cy, state->f.cy,
			  !state->f.p, state->f.p,
			  !state->f.s, state->f.s };

  switch (instr_bits) {
  case 0: //RET
    if (conditions[cond_bits] || extra_bit) {
      state->pc = (uint16_t) (state->memory[state->sp + 1] << 8) | state->memory[state->sp];
      state->sp += 2;
    }
    break;
  case 1: //JMP
    if (conditions[cond_bits] || extra_bit) {
      state->pc = (uint16_t) (high << 8) | low;
    } else {
      state->pc += 2;
    }
    break;
  case 2: //CALL
    //debug
    /*if (((opcode[2] << 8) | opcode[1]) == 0x0005) {
      if (state->c == 9) { //cp/m print routine
	uint16_t offset = (uint16_t) (state->d << 8) | state->e;
	char* str = (char*) &state->memory[offset+3];
	while (*str != '$')
	  printf("%c", *str++);
	printf("\n");
	state->pc += 2;
      } else if (state->c == 2) {
	printf("print char routine called\n");
	state->pc += 2;
      }
    } else if (((opcode[2] << 8) | opcode[1]) == 0) {
      state->pc += 2;
      exit(EXIT_SUCCESS);
      } else */{
      if (conditions[cond_bits] || extra_bit) {
	mem_write(state, (uint8_t) (ret >> 8) & 0xff, state->sp - 1);
	mem_write(state, (uint8_t) ret & 0xff, state->sp - 2);
	state->sp -= 2;
	state->pc = (uint16_t) (high << 8) | low;
      } else {
	state->pc += 2;
      }
    }
    break;
  case 3: //RST
    mem_write(state, (uint8_t) (rst_ret >> 8) & 0xff, state->sp - 1);
    mem_write(state, (uint8_t) rst_ret & 0xff, state->sp - 2);
    state->sp -= 2;
    state->pc = (uint16_t) (cond_bits << 3) & 0b0000000000111000;
    //the same bits for condition are used as vector in rst
    break;
  default:
    printf("<ERROR> invalid jmp/call/ret/rst instruction\n");
    exit(EXIT_FAILURE);
    break;
  }
}

void mem_write(system_state* state, uint8_t byte, uint16_t address) {
  if (address < 0x2000) {
    printf("<INFO> write to ROM attempted (%d)\n", address);
    return;
  }

  if (address < 0x4000 && address > 0x23ff) {
    state->vram_changed = true;
  }

  state->memory[address] = byte;
}

bool eval_opcode(system_state* state) {
  unsigned char* opcode = &state->memory[state->pc];
  //disas_opcode(state->memory, state->pc);
  //printf("\tC=%d,P=%d,S=%d,Z=%d\n", state->f.cy, state->f.p, state->f.s, state->f.z);
  //printf("\tA $%02x B $%02x C $%02x D $%02x E $%02x H $%02x L $%02x SP $%04x\n", state->a, state->b, state->c, state->d, state->e, state->h, state->l, state->sp);
  state->pc++;
  
  switch (*opcode) {
  //8 bit move/store/load
  case 0x02: //STAX B
    mem_write(state, state->a, (uint16_t) (state->b << 8) | state->c);
    break;
  case 0x06: //MVI B, d8
    mov_8_imm_reg(&state->b, opcode[1], state);
    break;
  case 0x0a: //LDAX B
    state->a = state->memory[(state->b << 8) | state->c];
    break;
  case 0x0e: //MVI C, d8
    mov_8_imm_reg(&state->c, opcode[1], state);
    break;
  case 0x12: //STAX D
    mem_write(state, state->a, (uint16_t) (state->d << 8) | state->e);
    break;
  case 0x16: //MVI D, d8
    mov_8_imm_reg(&state->d, opcode[1], state);
    break;
  case 0x1a: //LDAX D
    state->a = state->memory[(state->d << 8) | state->e];
    break;
  case 0x1e: //MVI E, d8
    mov_8_imm_reg(&state->e, opcode[1], state);
    break;
  case 0x26: //MVI H, d8
    mov_8_imm_reg(&state->h, opcode[1], state);
    break;
  case 0x2e: //MVI L, d8
    mov_8_imm_reg(&state->l, opcode[1], state);
    break;
  case 0x32: //STA a16
    mem_write(state, state->a, (uint16_t) (opcode[2] << 8) | opcode[1]);
    state->pc += 2; //3 bytes instruction
    break;
  case 0x36: //MVI M, d8
    mov_8_imm_mem(state, opcode[1]);
    break;
  case 0x3a: //LDA a16
    state->a = state->memory[(opcode[2] << 8) | opcode[1]];
    state->pc += 2; //3 byte instruction
    break;
  case 0x3e: //MVI A, d8
    mov_8_imm_reg(&state->a, opcode[1], state);
    break;
  //MOV reg-reg instructions
  //MOV is coded as 01[3 bit dest][3 bit src]
  //dest and src count up b,c,d,e,h,l,m,a in binary
  //to mask out dest & with 0x38
  //to mask out src & with 0x07
  case 0x40:
  case 0x41:
  case 0x42:
  case 0x43:
  case 0x44:
  case 0x45:
  case 0x46:
  case 0x47:
  case 0x48:
  case 0x49:
  case 0x4a:
  case 0x4b:
  case 0x4c:
  case 0x4d:
  case 0x4e:
  case 0x4f:
  case 0x50:
  case 0x51:
  case 0x52:
  case 0x53:
  case 0x54:
  case 0x55:
  case 0x56:
  case 0x57:
  case 0x58:
  case 0x59:
  case 0x5a:
  case 0x5b:
  case 0x5c:
  case 0x5d:
  case 0x5e:
  case 0x5f:
  case 0x60:
  case 0x61:
  case 0x62:
  case 0x63:
  case 0x64:
  case 0x65:
  case 0x66:
  case 0x67:
  case 0x68:
  case 0x69:
  case 0x6a:
  case 0x6b:
  case 0x6c:
  case 0x6d:
  case 0x6e:
  case 0x6f:
  case 0x70:
  case 0x71:
  case 0x72:
  case 0x73:
  case 0x74:
  case 0x75:
  case 0x77:
  case 0x78:
  case 0x79:
  case 0x7a:
  case 0x7b:
  case 0x7c:
  case 0x7d:
  case 0x7e:
  case 0x7f:
    {
      uint16_t hl_ptr = (uint16_t) (state->h << 8) | state->l;
      uint8_t* regarray[] = {
			     &state->b, &state->c,
			     &state->d, &state->e,
			     &state->h, &state->l,
			     &state->memory[hl_ptr], &state->a };
      uint8_t* dest,* src;
      uint8_t destbits, srcbits;
      destbits = (*opcode & 0x38) >> 3; //0b00111000
      srcbits = *opcode & 0x07; //00000111
      dest = regarray[destbits];
      src = regarray[srcbits];

      if (destbits == 6) { //memory
	mem_write(state, *src, hl_ptr);
      } else {
	mov_8_reg_reg(dest, src);
      }
      break;
    }
  //16 bit load/store/move
  case 0xc5:
  case 0xd5:
  case 0xe5:
  case 0xf5:
    //PUSH
    {
      uint8_t flags = (uint8_t) ((state->f.s != 0) << 7)		\
	| ((state->f.z != 0) << 6) | 0 |				\
	((state->f.ac != 0) << 4) | 0 |					\
				       ((state->f.p != 0) << 2) | 0b10 |	\
	(state->f.cy != 0);
      uint8_t* rp[4][2] = {
			 {&state->b, &state->c},
			 {&state->d, &state->e},
			 {&state->h, &state->l},
			 {&state->a, &flags} };
      uint8_t rp_bits = (*opcode & 0x35) >> 4; //11xx0101
      mem_write(state, *rp[rp_bits][0], state->sp - 1);
      mem_write(state, *rp[rp_bits][1], state->sp - 2);
      state->sp -= 2;
      break;
    }
  case 0xc1:
  case 0xd1:
  case 0xe1:
  case 0xf1:
    //POP
    {
      uint8_t* rp[3][2] = {
			   {&state->b, &state->c},
			   {&state->d, &state->e},
			   {&state->h, &state->l} };
      uint8_t rp_bits = (*opcode & 0x35) >> 4; //11xx0001
      if (rp_bits == 0b11) {
	uint8_t flags = state->memory[state->sp];
	state->f.s = (flags & 0b10000000) >> 7;
	state->f.z = (flags & 0b01000000) >> 6;
	state->f.ac = (flags & 0b00010000) >> 4;
	state->f.p = (flags & 0b00000100) >> 2;
	state->f.cy = (flags & 0b00000001);
	state->a = state->memory[state->sp + 1];
	state->sp += 2;
	break;
      }
      *rp[rp_bits][1] = state->memory[state->sp];
      *rp[rp_bits][0] = state->memory[state->sp + 1];
      state->sp += 2;
      break;
    }
  case 0x01:
  case 0x11:
  case 0x21:
  case 0x31:
    //LXI r, d16
    {
      uint8_t rp_bits = (*opcode & 0x30) >> 4; //00xx0001
      switch (rp_bits) {
      case 0:
	state->b = opcode[2];
	state->c = opcode[1];
	break;
      case 1:
	state->d = opcode[2];
	state->e = opcode[1];
	break;
      case 2:
	state->h = opcode[2];
	state->l = opcode[1];
	break;
      case 3:
	state->sp = (uint16_t) (opcode[2] << 8) | opcode[1];
	break;
      default:
	printf("<ERROR> invalid register pair in LXI\n");
	exit(EXIT_FAILURE);
	break;
      }
      state->pc += 2; //3 byte instruction
      break;
    }
  case 0x22:
    //SHLD a16
    mem_write(state, state->l, (uint16_t) (opcode[2] << 8) | opcode[1]);
    mem_write(state, state->h, (uint16_t) ((opcode[2] << 8) | opcode[1]) + 1);
    state->pc += 2; //3 byte instruction
    break;
  case 0x2a:
    //LHLD a16
    state->l = state->memory[(opcode[2] << 8) | opcode[1]];
    state->h = state->memory[((opcode[2] << 8) | opcode[1]) + 1];
    state->pc += 2; //3 byte instruction
    break;
  case 0xe3:
    //XTHL
    {
      uint8_t tmp = state->h;
      state->h = state->memory[state->sp + 1];
      mem_write(state, tmp, state->sp + 1);
      tmp = state->l;
      state->l = state->memory[state->sp];
      mem_write(state, tmp, state->sp);
      break;
    }
  case 0xf9:
    //SPHL
    state->sp = (uint16_t) (state->h << 8) | state->l;
    break;
  case 0xeb:
    //XCHG
    {
      uint8_t tmp = state->h;
      state->h = state->d;
      state->d = tmp;
      tmp = state->l;
      state->l = state->e;
      state->e = tmp;
      break;
    }
  //jumps and calls
  case 0xe9:
    //PCHL
    state->pc = (uint16_t) (state->h << 8) | state->l;
    break;
  case 0xc2:
  case 0xc8:
  case 0xc9:
  case 0xd2:
  case 0xe2:
  case 0xf2:
  case 0xc0:
  case 0xc3:
  case 0xca:
  case 0xda:
  case 0xd8:
  case 0xea:
  case 0xe0:
  case 0xe8:
  case 0xfa:
  case 0xcb:
  case 0xc4:
  case 0xd4:
  case 0xe4:
  case 0xf0:
  case 0xf4:
  case 0xf8:
  case 0xcc:
  case 0xd0:
  case 0xdc:
  case 0xec:
  case 0xfc:
  case 0xcd:
  case 0xdd:
  case 0xed:
  case 0xfd:
      if (*opcode == 0xcb)
	printf("<INFO> alternate JMP used\n");
      if ((*opcode & 0x0f) == 0x0d && (*opcode & 0xf0) != 0xc0)
	printf("<INFO> alternate call instruction used\n");
      if (*opcode == 0xd9)
	printf("<INFO> alternate RET used\n");
      ret_jmp_call_rst(opcode, opcode[1], opcode[2], state);
      break;
  case 0x03:
  case 0x13:
  case 0x23:
  case 0x33:
    //INX
    {
      uint16_t tmp;
      uint8_t rp_bits = (*opcode & 0x30) >> 4;
      uint8_t* rp[][2] = {
			  {&state->b, &state->c},
			  {&state->d, &state->e},
			  {&state->h, &state->l} };
      switch (rp_bits) {
      case 0:
      case 1:
      case 2:
	tmp = (uint16_t) ((*rp[rp_bits][0] << 8) | *rp[rp_bits][1]);
	tmp++;
	*rp[rp_bits][0] = (uint8_t) (tmp >> 8) & 0x00ff;
	*rp[rp_bits][1] = (uint8_t) tmp & 0x00ff;
	break;
      case 3:
	state->sp++;
	break;
      default:
	printf("<ERROR> invalid register pair in INX\n");
	exit(EXIT_FAILURE);
	break;
      }
      break;
    }
  case 0x0b:
  case 0x1b:
  case 0x2b:
  case 0x3b:
    //DCX
    {
      uint16_t tmp;
      uint8_t rp_bits = (*opcode & 0x30) >> 4;
      uint8_t* rp[][2] = {
			  {&state->b, &state->c},
			  {&state->d, &state->e},
			  {&state->h, &state->l} };
      switch (rp_bits) {
      case 0:
      case 1:
      case 2:
	tmp = (uint16_t) ((*rp[rp_bits][0] << 8) | *rp[rp_bits][1]);
	tmp--;
	*rp[rp_bits][0] = (uint8_t) (tmp >> 8) & 0x00ff;
	*rp[rp_bits][1] = (uint8_t) tmp & 0x00ff;
	break;
      case 3:
	state->sp--;
	break;
      default:
	printf("<ERROR> invalid register pair in DCX\n");
	exit(EXIT_FAILURE);
	break;
      }
      break;
    }
  case 0x09:
  case 0x19:
  case 0x29:
  case 0x39:
    //DAD
    {
      uint32_t result; //higher precision to capture carry
      uint8_t rp_bits = (*opcode & 0x30) >> 4;
      uint8_t* rp[][2] = {
			  {&state->b, &state->c},
			  {&state->d, &state->e},
			  {&state->h, &state->l} };
      switch (rp_bits) {
      case 0:
      case 1:
      case 2:
	result = (uint32_t) ((state->h << 8) | state->l) + (uint32_t) ((*rp[rp_bits][0] << 8) | *rp[rp_bits][1]);
	state->f.cy = (result > 0xffff);
	result &= 0xffff;
	state->h = (uint8_t) (result >> 8) & 0x00ff;
	state->l = (uint8_t) result & 0x00ff;
	break;
      case 3:
	result = (uint32_t) ((state->h << 8) | state->l) + state->sp;
	state->f.cy = (result > 0xffff);
	result &= 0xffff;
	state->h = (uint8_t) (result >> 8) & 0x00ff;
	state->l = (uint8_t) result & 0x00ff;
	break;
      default:
	printf("<ERROR> invalid register pair in DAD\n");
	exit(EXIT_FAILURE);
	break;
      }
      break;
    }
  case 0x80:
  case 0x81:
  case 0x82:
  case 0x83:
  case 0x84:
  case 0x85:
  case 0x86:
  case 0x87:
  case 0x88:
  case 0x89:
  case 0x8a:
  case 0x8b:
  case 0x8c:
  case 0x8d:
  case 0x8e:
  case 0x8f:
  case 0x90:
  case 0x91:
  case 0x92:
  case 0x93:
  case 0x94:
  case 0x95:
  case 0x96:
  case 0x97:
  case 0x98:
  case 0x99:
  case 0x9a:
  case 0x9b:
  case 0x9c:
  case 0x9d:
  case 0x9e:
  case 0x9f:
  case 0xa0:
  case 0xa1:
  case 0xa2:
  case 0xa3:
  case 0xa4:
  case 0xa5:
  case 0xa6:
  case 0xa7:
  case 0xa8:
  case 0xa9:
  case 0xaa:
  case 0xab:
  case 0xac:
  case 0xad:
  case 0xae:
  case 0xaf:
  case 0xb0:
  case 0xb1:
  case 0xb2:
  case 0xb3:
  case 0xb4:
  case 0xb5:
  case 0xb6:
  case 0xb7:
  case 0xb8:
  case 0xb9:
  case 0xba:
  case 0xbb:
  case 0xbc:
  case 0xbd:
  case 0xbe:
  case 0xbf:
    //ADD ADC SUB SBB ANA XRA ORA CMP
    {
      uint16_t result; //extra precision to get carry bit
      uint8_t instr_bits = (*opcode & 0x38) >> 3;
      uint8_t reg_bits = (*opcode & 0x07);
      uint16_t hl_ptr = (uint16_t) (state->h << 8) | state->l;
      uint8_t* reg[] = {
			&state->b, &state->c, &state->d, &state->e,
			&state->h, &state->l, &state->memory[hl_ptr],
			&state->a};
      switch (instr_bits) {
      case 0: //ADD
	result = (uint16_t) state->a + (uint16_t) *reg[reg_bits];
	state->f.cy = (result > 0xff);
	state->f.z = ((result & 0xff) == 0);
	state->f.s = ((result & 0x80) != 0);
	state->f.p = parity8(result & 0xff);
	//TODO: ac
	state->a = (uint8_t) result & 0xff;
	break;
      case 1: //ADC
	result = (uint16_t) state->a + (uint16_t) *reg[reg_bits] + (uint16_t) state->f.cy;
	state->f.cy = (result > 0xff);
	state->f.z = ((result & 0xff) == 0);
	state->f.s = ((result & 0x80) != 0);
	state->f.p = parity8(result & 0xff);
	//TODO: ac
	state->a = (uint8_t) result & 0xff;
	break;
      case 2: //SUB
	result = (uint16_t) state->a + (uint16_t) (~*reg[reg_bits]) + 1;
	state->f.cy = !(result > 0xff);
	state->f.z = ((result & 0xff) == 0);
	state->f.s = ((result & 0x80) != 0);
	state->f.p = parity8(result & 0xff);
	//TODO: ac
	state->a = (uint8_t) result & 0xff;
	break;
      case 3: //SBB
	result = (uint16_t) state->a + (uint16_t) ~(*reg[reg_bits] + state->f.cy) + 1;
	state->f.cy = !(result > 0xff);
	state->f.z = ((result & 0xff) == 0);
	state->f.s = ((result & 0x80) != 0);
	state->f.p = parity8(result & 0xff);
	//TODO: ac
	state->a = (uint8_t) result & 0xff;
	break;
      case 4: //ANA
	result = (uint16_t) state->a & (uint8_t) *reg[reg_bits];
	state->f.cy = 0;
	state->f.z = ((result & 0xff) == 0);
	state->f.s = ((result & 0x80) != 0);
	state->f.p = parity8(result & 0xff);
	state->a = (uint8_t) result & 0xff;
	break;
      case 5: //XRA
	result = (uint8_t) state->a ^ (uint8_t) *reg[reg_bits];
	state->f.cy = 0;
	state->f.z = ((result & 0xff) == 0);
	state->f.s = ((result & 0x80) != 0);
	state->f.p = parity8(result & 0xff);
	state->a = (uint8_t) result & 0xff;
	break;
      case 6: //ORA
	result = (uint8_t) state->a | (uint8_t) *reg[reg_bits];
	state->f.cy = 0;
	state->f.z = ((result & 0xff) == 0);
	state->f.s = ((result & 0x80) != 0);
	state->f.p = parity8(result & 0xff);
	state->a = (uint8_t) result & 0xff;
	break;
      case 7: //CMP
	result = (uint16_t) state->a + (uint16_t) (~*reg[reg_bits]) + 1;
	state->f.cy = !(result > 0xff);
	state->f.z = ((result & 0xff) == 0);
	state->f.s = ((result & 0x80) != 0);
	state->f.p = parity8(result & 0xff);
	//TODO: ac
	break;
      default:
	printf("<ERROR> invalid instruction in 8 bit register/memory to accumulator opcode\n");
	exit(EXIT_FAILURE);
	break;
      }
      break;
    }
  case 0x07:
  case 0x0f:
  case 0x17:
  case 0x1f:
    //rotate accumulator instructions
    {
      uint8_t instr_bits = (*opcode & 0x18) >> 3;
      switch (instr_bits) {
      case 0: //RLC
	state->f.cy = (state->a & 0x80) >> 7; //get high bit
	state->a = (uint8_t) (state->a << 1);
	state->a |= state->f.cy;
	break;
      case 1: //RRC
	state->f.cy = (state->a & 1);
	state->a = (uint8_t) ((state->a & 1) << 7) | (uint8_t) (state->a >> 1);
	break;
      case 2: //RAL
	{
	  uint8_t tmp = state->f.cy;
	  state->f.cy = (state->a & 0x80) >> 7;
	  state->a = (uint8_t) (state->a << 1);
	  state->a |= tmp;
	  break;
	}
      case 3: //RAR
	{
	  uint8_t tmp = (uint8_t) (state->f.cy << 7);
	  state->f.cy = (state->a & (uint8_t) 0x01);
	  state->a = state->a >> 1;
	  state->a |= tmp;
	  break;
	}
      default:
	printf("<ERROR> invalid operation in rotate instruction\n");
	exit(EXIT_FAILURE);
	break;
      }
      break;
    }
  case 0xc6:
  case 0xd6:
  case 0xe6:
  case 0xf6:
  case 0xce:
  case 0xde:
  case 0xee:
  case 0xfe:
    //immediate arithmetic
    {
      uint16_t result; //extra precision to get carry bit
      uint8_t instr_bits = (*opcode & 0x38) >> 3;
      switch (instr_bits) {
      case 0: //ADI
	result = (uint16_t) state->a + (uint16_t) opcode[1];
	state->f.cy = (result > 0xff);
	state->f.z = ((result & 0xff) == 0);
	state->f.s = ((result & 0x80) != 0);
	state->f.p = parity8(result & 0xff);
	//TODO: ac
	state->a = (uint8_t) result & 0xff;
	break;
      case 1: //ACI
	result = (uint16_t) state->a + (uint16_t) opcode[1] + (uint16_t) state->f.cy;
	state->f.cy = (result > 0xff);
	state->f.z = ((result & 0xff) == 0);
	state->f.s = ((result & 0x80) != 0);
	state->f.p = parity8(result & 0xff);
	//TODO: ac
	state->a = (uint8_t) result & 0xff;
	break;
      case 2: //SUI
	result = (uint16_t) state->a + (uint16_t) (~opcode[1] + 1);
	state->f.cy = (result > 0xff);
	state->f.z = ((result & 0xff) == 0);
	state->f.s = ((result & 0x80) != 0);
	state->f.p = parity8(result & 0xff);
	//TODO: ac
	state->a = (uint8_t) result & 0xff;
	break;
      case 3: //SBI
	result = (uint16_t) state->a + (uint16_t) (~(opcode[1] + state->f.cy) + 1);
	state->f.cy = (result > 0xff);
	state->f.z = ((result & 0xff) == 0);
	state->f.s = ((result & 0x80) != 0);
	state->f.p = parity8(result & 0xff);
	//TODO: ac
	state->a = (uint8_t) result & 0xff;
	break;
      case 4: //ANI
	result = state->a & opcode[1];
	state->f.cy = 0;
	state->f.z = ((result & 0xff) == 0);
	state->f.s = ((result & 0x80) != 0);
	state->f.p = parity8(result & 0xff);
	state->a = (uint8_t) result & 0xff;
	break;
      case 5: //XRI
	result = (uint8_t) state->a ^ (uint8_t) opcode[1];
	state->f.cy = 0;
	state->f.z = ((result & 0xff) == 0);
	state->f.s = ((result & 0x80) != 0);
	state->f.p = parity8(result & 0xff);
	state->a = (uint8_t) result & 0xff;
	break;
      case 6: //ORI
	result = (uint8_t) state->a | (uint8_t) opcode[1];
	state->f.cy = 0;
	state->f.z = ((result & 0xff) == 0);
	state->f.s = ((result & 0x80) != 0);
	state->f.p = parity8(result & 0xff);
	state->a = (uint8_t) result & 0xff;
	break;
      case 7: //CPI
	result = (uint16_t) state->a + (uint16_t) (~opcode[1] + 1);
	state->f.cy = (result > 0xff);
	state->f.z = ((result & 0xff) == 0);
	state->f.s = ((result & 0x80) != 0);
	state->f.p = parity8(result & 0xff);
	//TODO: ac
	break;
      default:
	printf("<ERROR> invalid instruction in 8 bit immediate arithmetic opcode\n");
	exit(EXIT_FAILURE);
	break;
      }
      state->pc++;
      break;
    }
  case 0x3f: //CMC
    state->f.cy = !state->f.cy;
    break;
  case 0x37: //STC
    state->f.cy = true;
    break;
  case 0x04:
  case 0x14:
  case 0x24:
  case 0x34:
  case 0x0c:
  case 0x1c:
  case 0x2c:
  case 0x3c:
    //INR
    {
      uint8_t result;
      uint8_t reg_bits = (*opcode & 0x38) >> 3;
      uint16_t hl_ptr = (uint16_t) (state->h << 8) | state->l;
      uint8_t* reg[] = {
			&state->b, &state->c, &state->d, &state->e, &state->h, &state->l,
			&state->memory[hl_ptr], &state->a};
      result = *reg[reg_bits] + 1;
      state->f.z = (result == 0);
      state->f.s = ((result & 0x80) != 0);
      state->f.p = parity8(result);
      //todo: ac
      if (reg_bits == 6) { //memory
	mem_write(state, result, hl_ptr);
      } else {
	*reg[reg_bits] = result;
      }
      break;
    }
  case 0x05:
  case 0x15:
  case 0x25:
  case 0x35:
  case 0x0d:
  case 0x1d:
  case 0x2d:
  case 0x3d:
    //DCR
    {
      uint8_t result;
      uint8_t reg_bits = (*opcode & 0x38) >> 3;
      uint16_t hl_ptr = (uint16_t) (state->h << 8) | state->l;
      uint8_t* reg[] = {
			&state->b, &state->c, &state->d, &state->e, &state->h, &state->l,
			&state->memory[hl_ptr], &state->a};
      result = *reg[reg_bits] - 1;
      state->f.z = (result == 0);
      state->f.s = ((result & 0x80) != 0);
      state->f.p = parity8(result);
      //todo: ac
      if (reg_bits == 6) { //memory
	mem_write(state, result, hl_ptr);
      } else {
	*reg[reg_bits] = result;
      }
      break;
    }
  case 0x2f: //CMA
    state->a = ~state->a;
    break;
  case 0x27: //DAA
    printf("<INFO> DAA not currently implemented\n");
    break;
  case 0x10:
  case 0x20:
  case 0x30:
  case 0x08:
  case 0x18:
  case 0x28:
  case 0x38:
    //NOP
    printf("<INFO> alternate NOP used\n");
  case 0x00:
    //do nothing 4 cycles
    break;
  case 0xfb: //EI
    state->ime = true;
    break;
  case 0xf3: //DI
    state->ime = false;
    break;
  case 0xdb: //IN
    state->pc++; //2 byte instruction
    break;
  case 0xd3: //OUT
    state->pc++; //2 byte instruction
    break;
  case 0x76: //HLT
    return true; //todo: use proper behavior once interrupts are working
    break;
  default:
    unimplemented_instr(state);
    break;
  }
  state->f.cy = (state->f.cy) ? true : false; //make sure carry is only 0 or 1
  //otherwise math instructions will be inaccurate
  //DEBUG
  //getchar();
  return false; //not done
}
