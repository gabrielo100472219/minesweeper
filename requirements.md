# Minesweep

Project idea to learn rust. Build a minesweep terminal app.

## Requirements

### General

- Game displays a grid [ ] made up of these
- Player can open a square or mark it for bombs

### Initial state

- At first, all grid squares are empty.
- When player opens a square, bombs are generated and placed randomly

### Gameplay

- Player moves between squares with hjkl
- Player opens square with f
- Player marks square with d

### End

- If player opens square with bomb, end game.
- If player opens all empty squares, end game.

### Other

- When player opens a square, all adjacent non-mined cells open automatically. Do this when there are no adjacent bombs, and open all squares adjacent to 0 bombs until arriving to the surrounding cells adjacent to 1 or more, including them.
- Non mined cells display the number of bombs they are adjacent to.
- Optional, adding animations.
