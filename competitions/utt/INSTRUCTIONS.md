# Ultimate Tic-Tac-Toe competition

## The rules of UTT

TODO

## Agent input/output

Note: in this competition every command ends with a newline which is included here as `\n`.

The tile indexing system is broken into two parts: the grid and then the tile within that grid.
These two numbers should be in the range 0-8 (inclusive) such that if you want to index the middle tile in the top left grid you would give the coordinates: `0` (grid) `4`.
You can see a diagram below:

| 0    | 1    | 2    |
| 3    | 4    | 5    |
| 6    | 7    | 8    |



At the start of the game your agent will receive an input of either `S R\n` indicating that you are the Red player or `S B\n` indicating you are the blue player.
The red player goes first.

Whenever it is your turn you will receive a "Request Action" (see below) command which will list all the grids that are currently playable. Then you must output the coordinates in the format `M {GRID} {TILE}\n` .

When either you or the other player has made a move you will receive a "Tile Placed" event (including for you own tiles) and if it caused a grid to be won (or stalemated) then a "Grid Won" event



### Input

#### Request Action

Format:

`R [COMMA SEPARATED LIST OF PLAYABLE GRIDS]\n`

e.g. `R 1\n` (indicating you can only play in grid 1 - the top middle grid).

e.g. `R 1,2,3,8\n` (indicating you can only play in grids 1, 2, 3 or 8)

#### Tile Placed

`P {R or B - the player who placed the tile} {GRID} {TILE}\n`

e.g. `P R 4 1` the red player (who could be you) placed a tile at the top middle location in the absolute middle grid.

#### Grid Won

`G {R or B or S - the player who won the grid} {GRID}\n`

`S` means stalemate, which occurs when it is no longer possible for either player to win the grid. At this point it counts as a won grid in the sense that no more tiles can be placed in it.
