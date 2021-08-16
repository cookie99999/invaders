#ifndef _INVADERS_H
#define _INVADERS_H

#include <stdint.h>
#include "8080.h"

uint8_t port_in(system_state* state, uint8_t port);
void port_out(system_state* state, uint8_t port);
void fire_interrupt(system_state* state, uint8_t vector);
#endif
