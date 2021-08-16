#ifndef _EVAL_H
#define _EVAL_H

#include <stdbool.h>
#include "8080.h"

bool eval_opcode(system_state* state);
void mem_write(system_state* state, uint8_t byte, uint16_t address);
#endif
