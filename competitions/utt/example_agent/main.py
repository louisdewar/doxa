# An example agent for UTTT competition.
# It's designed to keep playing valid moves without any thought to strategy beyond that

import sys, random

# Runs at startup to determine which player the current user is
def get_player():
    first_input = input()
    if first_input == "S R":
        return "R"
    elif first_input == "S B":
        return "B"
    else:
        raise Exception("The first message should be `S R` or `S B` got `{}`".format(first_input))

def main():
    # Store the owners of individual tiles
    main_grid = [[None for _ in range(0,9)] for _ in range(0, 9)]

    current_player = get_player()

    while True:
        # Get the next line of input and then split it at each space
        parts = input().split(" ")
        # Request Action
        if parts[0] == "R":
            playable_grids = [int(grid_pos) for grid_pos in parts[1].split(",")]

            # For each playable grid find the tiles which are currently free
            possible_moves = [{ 'grid': grid_pos, 'tile': tile_pos } for grid_pos in playable_grids for tile_pos in range(0, 9) if main_grid[grid_pos][tile_pos] is None]

            move = random.choice(possible_moves)

            print("M {} {}".format(move['grid'], move['tile']))
        # Tile placed
        elif parts[0] == "P":
            player = parts[1]
            grid_pos = int(parts[2])
            tile_pos = int(parts[3])
            main_grid[grid_pos][tile_pos] = player
        # Grid won
        elif parts[0] == "G":
            # Either R, B or S (for stalemate) depending on who won the grid
            winner = parts[1]
            # The grid position
            grid_pos = int(parts[2])
            # This agent does not care about the overall grids as it only cares about playable valid moves
            # in a real agent you will likely care about this though


main()
