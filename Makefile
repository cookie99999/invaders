CC := gcc
WARNINGS := -Wall -Wextra -pedantic -Wshadow -Wpointer-arith -Wcast-align \
            -Wwrite-strings -Wmissing-prototypes -Wmissing-declarations \
            -Wredundant-decls -Wnested-externs -Winline -Wno-long-long \
            -Wstrict-prototypes
CFLAGS := -O0 $(WARNINGS) --std=c2x -g

vpath %.c src
vpath %.h src

all: invaders emu8080 disas

debug: CFLAGS += -DDEBUG
debug: all

invaders: main.o invaders.o eval.o disas_core.o
	$(CC) -o $@ $^ `sdl2-config --cflags --libs` -lpthread
main.o: main.c invaders.h 8080.h eval.h
	$(CC) -c $< $(CFLAGS)
invaders.o: invaders.c invaders.h 8080.h eval.h
	$(CC) -c $< $(CFLAGS)

emu8080: cpu_main.o eval.o disas_core.o
	$(CC) -o $@ $^
eval.o: eval.c eval.h 8080.h
	$(CC) -c $< $(CFLAGS)
cpu_main.o: cpu_main.c eval.h 8080.h
	$(CC) -c $< $(CFLAGS)

disas: disas.o disas_core.o
	$(CC) -o $@ $^
disas%.o: disas%.c disas.h
	$(CC) -c $< $(CFLAGS)

clean:
	-@rm emu8080 disas $(wildcard *.o)

.PHONY: all clean
