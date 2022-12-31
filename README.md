# invaders
Space Invaders emulator

## Building
This project depends on SDL 2 and the standard library. It has only been tested on GNU/Linux. The CPU core passes all available tests, including the 8080EXM.COM.

## Running
The main program expects a file called "invaders.bin" in the same directory. This file is the result of concatenating (in this order) invaders.h, invaders.g, invaders.f, and invaders.e from the MAME set "invaders.zip".

## To do:
- Add sound
- Fix poor performance
