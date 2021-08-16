#include <stdint.h>
#include <stdio.h>
#include <stdbool.h>
#include "8080.h"
#include "invaders.h"

uint8_t port_in(system_state* state, uint8_t port) {
  switch (port) {
  case 0x00:
    return 0x01;
    break;
  case 0x01:
    return state->ports[1];
    break;
  case 0x03:
    {
      uint16_t result = (uint16_t) (state->shift1 << 8) | state->shift0;
      return ((result >> (8 - state->shift_offset)) & 0x00ff);
      break;
    }
  default:
    return 0x00;
    break;
  }
}

void port_out(system_state* state, uint8_t port) {
  switch (port) {
  case 0x02:
    state->shift_offset = (state->a & 0x07);
    break;
  case 0x04:
    state->shift0 = state->shift1;
    state->shift1 = state->a;
    break;
  default:
    break;
  }
}

void fire_interrupt(system_state* state, uint8_t vector) {
  if (!state->ime)
    return;
  
  state->memory[state->sp - 1] = (state->pc & 0xff00) >> 8;
  state->memory[state->sp - 2] = state->pc &0x00ff;
  state->sp -= 2;

  state->pc = 8 * vector;
  state->ime = false;
}
