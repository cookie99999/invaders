#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
//#include <threads.h>
#ifdef _WIN32
#include <SDL.h>
#else
#include <SDL2/SDL.h>
#endif
#include "8080.h"
#include "eval.h"
#include "disas.h"
#include "invaders.h"

#define MEMSIZE 0x4000
#define VRAM 0x2400
#define SCREEN_X 256
#define SCREEN_Y 224

void vram_to_rgba8888(uint8_t* vram, uint32_t* buffer);
uint32_t video_callback(uint32_t interval, void* state);
static system_state* init_state();

static system_state* init_state() {
  system_state* state = calloc(1, sizeof(system_state));

  if (!state) {
    printf("<ERROR> failed allocating %zu bytes\n", sizeof(system_state));
    exit(EXIT_FAILURE);
  }

  state->memory = calloc(1, MEMSIZE);

  if (!state->memory) {
    printf("<ERROR> failed allocating %d bytes\n", MEMSIZE);
    exit(EXIT_FAILURE);
  }

  state->last_interrupt = 2;
  state->ime = true;

  return state;
}

/*static int cpu_thread(void* _state) {
  system_state* restrict state = _state;
  bool done = false;

  while (!done) {
  uint8_t* opcode = &state->memory[state->pc];
  state->vram_changed = false;

  if (*opcode == 0xdb) { //IN
  uint8_t port = opcode[1];
  state->a = port_in(state, port);
  } else if (*opcode == 0xd3) { //OUT
  uint8_t port = opcode[1];
  port_out(state, port);
  }

  done = eval_opcode(state);
  }

  return 0;
  }*/

static void del_state(system_state* state) {
  free(state->memory);
  free(state);
}

int main(int argc, char** argv) {
  system_state* state = init_state();

  FILE* f = NULL;
  f = fopen("invaders.bin", "rb");

  if (!f) {
    printf("<ERROR> could not open invaders.bin\n");
    exit(EXIT_FAILURE);
  }

  fseek(f, 0L, SEEK_END);
  unsigned long fsize = (unsigned long)ftell(f);
  fseek(f, 0L, SEEK_SET);
  if (fsize > MEMSIZE) {
    printf("<ERROR> file too large for emulated memory\n");
    fclose(f);
    del_state(state);
    exit(EXIT_FAILURE);
  }
  fread(state->memory, fsize, 1, f);
  fclose(f);

  uint8_t* vram = &state->memory[VRAM];
  uint32_t* buffer = calloc(SCREEN_X * SCREEN_Y, sizeof(uint32_t));
  if (!buffer) {
    printf("<ERROR> failed allocating %d bytes\n", SCREEN_X * SCREEN_Y);
    exit(EXIT_FAILURE);
  }

  bool done = false;

  SDL_Window* window = NULL;
  SDL_Renderer* renderer = NULL;
  SDL_Texture* screentex = NULL;
  void* pixels = NULL;
  int pitch = 0;
  SDL_Event e;
  SDL_Color colors[2];
  colors[0].r = colors[0].g = colors[0].b = (uint8_t)0x00;
  colors[1].r = colors[1].g = colors[1].b = (uint8_t)0xff;
  if (0 > SDL_Init(SDL_INIT_VIDEO | SDL_INIT_TIMER)) {
    printf("<ERROR> SDL init failed: %s\n", SDL_GetError());
    exit(EXIT_FAILURE);
  }

  window = SDL_CreateWindow("Space Invaders", 0,
			    0, 224, 256, SDL_WINDOW_SHOWN);
  if (!window) {
    printf("<ERROR> could not create sdl window: %s\n", SDL_GetError());
    exit(EXIT_FAILURE);
  }

  renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED | SDL_RENDERER_TARGETTEXTURE);
  if (!renderer) {
    printf("<ERROR> couldn't create renderer\n");
    return EXIT_FAILURE;
  }

  screentex = SDL_CreateTexture(renderer, SDL_PIXELFORMAT_RGBA8888, SDL_TEXTUREACCESS_STREAMING, 224, 256);
  if (!screentex) {
    printf("<ERROR> couldn't create texture: %s\n", SDL_GetError());
    return EXIT_FAILURE;
  }

  SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
  SDL_RenderClear(renderer);

  SDL_AddTimer(16, video_callback, (void *)state);
  //thrd_t cpu;
  //thrd_create(cpu, cpu_thread, state);

  while (!done) {
    uint8_t* opcode = &state->memory[state->pc];

    if (*opcode == 0xdb) { //IN
      uint8_t port = opcode[1];
      state->a = port_in(state, port);
    } else if (*opcode == 0xd3) { //OUT
      uint8_t port = opcode[1];
      port_out(state, port);
    }
    
    done = eval_opcode(state);
    
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
	    if (0 > SDL_LockTexture(screentex, NULL, &pixels, &pitch)) {
	      printf("<ERROR> couldn't lock texture: %s\n", SDL_GetError());
	    }
	    vram_to_rgba8888(vram, (uint32_t *) pixels);
	    SDL_UnlockTexture(screentex);
	    pixels = NULL;
	    SDL_RenderCopy(renderer, screentex, NULL, NULL);
	    SDL_RenderPresent(renderer);
	  }
	  else {
	    fire_interrupt(state, 1);
	    state->last_interrupt = 1;
	  }
	}
	break;
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
  SDL_DestroyRenderer(renderer);
  free(buffer);
  SDL_DestroyWindow(window);
  SDL_Quit();
  del_state(state);
  return EXIT_SUCCESS;
}

void vram_to_rgba8888(uint8_t* vram, uint32_t* buffer) {
  for (int x = 0; x < SCREEN_X; x += 8) { //rows
    for (int y = 0; y < SCREEN_Y; y++) { //columns
      for (int k = 7; k >= 0; k--) { //bits
	int bufx = y;
	int bufy = -(x + k) + SCREEN_X - 1;
	int buf_offset = bufy * SCREEN_Y + bufx;
	int offset = (y * (SCREEN_X / 8)) + (x / 8);
	uint8_t pixel = (vram[offset] >> k) & 0x01;
	buffer[buf_offset] = pixel ? 0xFFFFFFFF : 0x000000FF;
      }
    }
  }
}

uint32_t video_callback(uint32_t interval, void* state) {
  SDL_Event e;

  e.type = SDL_USEREVENT;
  SDL_PushEvent(&e);

  return 8;
}
