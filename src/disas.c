#include <stdlib.h>
#include <stdio.h>
#include "disas.h"

int main(int argc, char** argv) {
	if (argc < 2) {
		printf("Usage: disas <file>\n");
		exit(EXIT_FAILURE);
	}

	FILE* f = NULL;
	f = fopen(argv[1], "rb");
	if (!f) {
		printf("<ERROR> could not open %s\n", argv[1]);
		exit(EXIT_FAILURE);
	}

	fseek(f, 0L, SEEK_END);
	unsigned long fsize = (unsigned long)ftell(f);
	fseek(f, 0L, SEEK_SET);

	unsigned char* buffer = malloc(fsize);
	if (!buffer) {
		printf("<ERROR> could not allocate %lu bytes for buffer\n", fsize);
		exit(EXIT_FAILURE);
	}

	if (!fread(buffer, fsize, 1, f)) {
		printf("<ERROR> could not read from file\n");
		exit(EXIT_FAILURE);
	}

	fclose(f);

	unsigned int pc = 0;

	while (pc < fsize) {
		pc += disas_opcode(buffer, pc);
	}

	return EXIT_SUCCESS;
}
