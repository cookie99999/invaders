#ifndef _8080_H
#define _8080_H

#include <stdint.h>
#include <stdbool.h>
//#include <stdatomic.h>

struct flags {
  uint8_t z : 1;
  uint8_t s : 1;
  uint8_t p : 1;
  uint8_t cy : 1;
  uint8_t ac : 1;
  uint8_t pad : 3;
};

typedef struct system_state {
  uint8_t a;
  uint8_t b;
  uint8_t c;
  uint8_t d;
  uint8_t e;
  uint8_t h;
  uint8_t l;
  uint16_t sp;
  uint16_t pc;
  uint8_t* memory;
  struct flags f;
  bool ime;
  uint8_t shift_offset;
  uint8_t shift0;
  uint8_t shift1;
  int type;
  unsigned long cyc;
  uint8_t last_interrupt;
  uint8_t ports[256];
} system_state;
#endif
