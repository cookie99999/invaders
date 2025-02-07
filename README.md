# invaders
Space Invaders emulator

## Building
Just needs cargo build, though I've only tested on GNU/Linux. The CPU passes all the 8080 tests I could find.

## Running
The main program expects a file called "invaders.bin" in the same directory. This file is the result of concatenating (in this order) invaders.h, invaders.g, invaders.f, and invaders.e from the MAME set "invaders.zip".

## To do:
- Fix sound, some effects play at the wrong times, and the looping effects don't work quite right
- Controller support
- Configuration options
