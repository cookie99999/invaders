# invaders
Space Invaders emulator

## Building
This project depends on SDL 2 and the standard library. It has only been tested on GNU/Linux.

## Running
The main program expects a file called "invaders.bin" in the same directory. This file is the result of concatenating (in this order) invaders.h, invaders.g, invaders.f, and invaders.e from the MAME set "invaders.zip".

## To do:
- Rotate video output correctly
- Fix bug where invaders won't die
- Add sound
- Fix poor performance
