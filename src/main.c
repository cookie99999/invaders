#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <SDL2/SDL.h>
#include "8080.h"
#include "eval.h"
#include "disas.h"
#include "invaders.h"

#define MEMSIZE 0x4000
#define VRAM 0x2400
#define SCREEN_X 256
#define SCREEN_Y 224

void vram_to_8bpp(uint8_t* vram, uint8_t* buffer);
uint32_t video_callback(uint32_t interval, system_state* state);

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

  state->last_interrupt = 2;
  state->ime = true;
  
  return state;
}

static void del_state(system_state* state) {
  free(state->memory);
  free(state);
}

int main() {
  system_state* state = init_state();

  FILE* f = fopen("invaders.bin", "rb");

  if (!f) {
    printf("<ERROR> could not open invaders.bin\n");
    exit(EXIT_FAILURE);
  }

  fseek(f, 0L, SEEK_END);
  unsigned long fsize = (unsigned long) ftell(f);
  fseek(f, 0L, SEEK_SET);

  fread(state->memory, fsize, 1, f);
  fclose(f);
  
  uint8_t* vram = &state->memory[VRAM];
  uint8_t* buffer = calloc(SCREEN_X * SCREEN_Y, 1);
  if (!buffer) {
    printf("<ERROR> failed allocating %d bytes\n", SCREEN_X * SCREEN_Y);
    exit(EXIT_FAILURE);
  }
  bool done = false;

  SDL_Window* window = NULL;
  SDL_Surface* surface = NULL;
  SDL_Surface* buf_surface = NULL;
  SDL_Event e;
  SDL_Color colors[2];
  colors[0].r = colors[0].g = colors[0].b = (uint8_t) 0x00;
  colors[1].r = colors[1].g = colors[1].b = (uint8_t) 0xff;
  if (0 > SDL_Init(SDL_INIT_VIDEO | SDL_INIT_TIMER)) {
    printf("<ERROR> SDL init failed: %s\n", SDL_GetError());
    exit(EXIT_FAILURE);
  }

  window = SDL_CreateWindow("Space Invaders", 0,
			    0, SCREEN_X, SCREEN_Y, SDL_WINDOW_SHOWN);
  if (!window) {
    printf("<ERROR> could not create sdl window: %s\n", SDL_GetError());
    exit(EXIT_FAILURE);
  }

  surface = SDL_GetWindowSurface(window);
  if (!surface) {
    printf("<ERROR> failed getting window surface: %s\n", SDL_GetError());
    exit(EXIT_FAILURE);
  }
  
  buf_surface = SDL_CreateRGBSurfaceFrom(buffer, SCREEN_X, SCREEN_Y, 8,
					 SCREEN_X, 0, 0, 0, 0);
  if (!buf_surface) {
    printf("<ERROR> failed creating rgb surface from buffer: %s\n", SDL_GetError());
    exit(EXIT_FAILURE);
  }
  
  if (0 > SDL_SetPaletteColors(buf_surface->format->palette, colors, 0, 2)) {
    printf("<ERROR> failed setting palette: %s\n", SDL_GetError());
    exit(EXIT_FAILURE);
  }
  SDL_FillRect(surface, NULL, SDL_MapRGB(surface->format, 0x00, 0x00, 0x00));
  SDL_UpdateWindowSurface(window);

  SDL_AddTimer(16, video_callback, (void*) state);
  while (!done) {
    uint8_t* opcode = &state->memory[state->pc];

    if (*opcode == 0xdb) { //IN
      uint8_t port = opcode[1];
      state->a = port_in(state, port);
    } else if (*opcode == 0xd3) { //OUT
      uint8_t port = opcode[1];
      port_out(state, port);
    }
    state->vram_changed = false;
    done = eval_opcode(state);
    if (state->vram_changed) {
      vram_to_8bpp(vram, buffer);
      if (0 > SDL_BlitSurface(buf_surface, NULL, surface, NULL)) {
	printf("<ERROR> failed to blit surface: %s\n", SDL_GetError());
	exit(EXIT_FAILURE);
      }
      if (0 > SDL_UpdateWindowSurface(window)) {
	printf("<ERROR> failed to update window: %s\n", SDL_GetError());
	exit(EXIT_FAILURE);
      }
    }
    while (SDL_PollEvent(&e) != 0) {
      switch (e.type) {
      case SDL_QUIT:
	done = true;
	break;
      case SDL_USEREVENT:
	if (state->ime) {
	  if (state->last_interrupt == 1) {
	    fire_interrupt(state, 2);
	    state->last_interrupt = 2;
	  } else {
	    fire_interrupt(state, 1);
	    state->last_interrupt = 1;
	  }
	}
      case SDL_KEYDOWN:
	switch (e.key.keysym.sym) {
	case SDLK_LEFT:
	  state->ports[1] |= 0b00100000;
	  break;
	case SDLK_RIGHT:
	  state->ports[1] |= 0b01000000;
	  break;
	case SDLK_c:
	  state->ports[1] |= 0b00000001;
	  break;
	case SDLK_RETURN:
	  state->ports[1] |= 0b00000100;
	  break;
	case SDLK_LCTRL:
	  state->ports[1] |= 0b00010000;
	  break;
	}
	break;
      case SDL_KEYUP:
	switch (e.key.keysym.sym) {
	case SDLK_LEFT:
	  state->ports[1] &= 0b11011111;
	  break;
	case SDLK_RIGHT:
	  state->ports[1] &= 0b10111111;
	  break;
	case SDLK_c:
	  state->ports[1] &= 0b11111110;
	  break;
	case SDLK_RETURN:
	  state->ports[1] &= 0b11111011;
	  break;
	case SDLK_LCTRL:
	  state->ports[1] &= 0b11101111;
	  break;
	}
	break;
      }
    }
  }
  SDL_FreeSurface(buf_surface);
  free(buffer);
  SDL_DestroyWindow(window);
  SDL_Quit();
  del_state(state);
  return EXIT_SUCCESS;
}

void vram_to_8bpp(uint8_t* vram, uint8_t* buffer) {
  for (int i = 0; i < SCREEN_Y; i++) { //rows
    for (int j = 0; j < SCREEN_X; j += 8) { //columns
      for (int k = 7; k >= 0; k--) { //bits
	int buf_offset = ((i * SCREEN_X) + j) + k;
	int offset = (i * (SCREEN_X/8)) + (j/8);
	uint8_t pixel = (vram[offset] >> k) & 0x01;
	buffer[buf_offset] = pixel;
      }
    }
  }
}

uint32_t video_callback(uint32_t interval, system_state* state) {
  SDL_Event e;
  
  e.type = SDL_USEREVENT;
  SDL_PushEvent(&e);
  
  return 8;
}
