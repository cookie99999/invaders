#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include "eval.h"
#include "disas.h"

#define MEMSIZE 0x4000

static system_state* init_state() {
  system_state* state = calloc(1, sizeof(system_state));

  if (!state) {
    printf("<ERROR> failed allocating %ld bytes\n", sizeof(system_state));
    exit(EXIT_FAILURE);
  }
  
  state->memory = malloc(MEMSIZE);

  if (!state->memory) {
    printf("<ERROR> failed allocating %d bytes\n", MEMSIZE);
    exit(EXIT_FAILURE);
  }
  
  return state;
}

static void del_state(system_state* state) {
  free(state->memory);
  free(state);
}

int main(int argc, char** argv) {
  if (argc != 2) {
    printf("Usage: emu8080 <file>\n");
    exit(EXIT_FAILURE);
  }

  bool done = false;
  system_state* state = init_state();
  
  FILE* f = fopen(argv[1], "rb");

  if (!f) {
    printf("<ERROR> could not open %s\n", argv[1]);
    exit(EXIT_FAILURE);
  }

  fseek(f, 0L, SEEK_END);
  unsigned long fsize = (unsigned long) ftell(f);
  fseek(f, 0L, SEEK_SET);

  fread(state->memory, fsize, 1, f);
  fclose(f);

  while (!done) {
    done = eval_opcode(state);
  }

  del_state(state);
  return EXIT_SUCCESS;
}

  
