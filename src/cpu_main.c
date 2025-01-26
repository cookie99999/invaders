#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include "eval.h"
#include "disas.h"

#define MEMSIZE 0x4000

static system_state* init_state() {
  system_state* state = calloc(1, sizeof(system_state));

  if (!state) {
    printf("<ERROR> failed allocating %lu bytes\n", sizeof(system_state));
    exit(EXIT_FAILURE);
  }

  state->memory = calloc(1, MEMSIZE);

  if (!state->memory) {
    printf("<ERROR> failed allocating %d bytes\n", MEMSIZE);
    exit(EXIT_FAILURE);
  }

  state->f.f1 = 1;

  return state;
}

static void del_state(system_state* state) {
  free(state->memory);
  free(state);
}

int main(int argc, char** argv) {
  if (argc < 2) {
    printf("Usage: emu8080 <file> [--cpm]\n");
    exit(EXIT_FAILURE);
  }

  bool done = false;
  system_state* state = init_state();

  FILE* f = NULL;
  f = fopen(argv[1], "rb");

  if (!f) {
    printf("<ERROR> could not open %s\n", argv[1]);
    exit(EXIT_FAILURE);
  }

  fseek(f, 0L, SEEK_END);
  unsigned long fsize = (unsigned long)ftell(f);
  fseek(f, 0L, SEEK_SET);

  //todo: make sure input file fits in allocated memory
  if (argc > 2) {
    if (!strcmp(argv[2], "--cpm")) {
      fread(&state->memory[0x100], fsize, 1, f);
      state->pc = 0x100;
      state->type = 1; //todo: switch to enum

      FILE* stub = NULL;
      stub = fopen("cpmstub.bin", "rb");
      if (!stub) {
	printf("<ERROR> could not open cpmstub.bin\n");
	exit(EXIT_FAILURE);
      }
      fseek(stub, 0L, SEEK_END);
      unsigned long stubsz = (unsigned long)ftell(stub);
      fseek(stub, 0L, SEEK_SET);
      fread(&state->memory[0xdc00], stubsz, 1, stub);
      state->memory[5] = 0xc3;
      state->memory[6] = 0x00;
      state->memory[7] = 0xdc; //JMP 0xdc00
    }
  }
  else {
    fread(state->memory, fsize, 1, f);
  }
  fclose(f);
  
  while (!done) {
    done = eval_opcode(state);
    //getchar();
  }

  del_state(state);
  return EXIT_SUCCESS;
}


