import random
from typing import List, Optional, Tuple

#########################################################
#                                                       #
#   YOU SHOULD NOT NEED TO EDIT THIS FILE AT ALL        #
#   UNLESS YOU ARE IMPLEMENTING A MORE ADVANCED AGENT   #
#                                                       #
#########################################################


class BaseAgent:
    def __init__(self) -> None:
        self.player = None
        self.opponent = None

    def set_player(self, player: str) -> None:
        """Sets the current player and opponent.

        Args:
            player (str): Either R for red or B for blue.
        """

        self.player = player
        self.opponent = "B" if player == "R" else "B"

    def make_move(
        self,
        boards: List[List[Optional[str]]],
        board_winners: List[Optional[str]],
        playable_boards: List[int],
    ) -> Tuple[int, int]:
        """Makes a move.

        Args:
            boards (List[List[Optional[str]]]): A list of local boards, which together form the global board.
                                                Each local board is a list of nine tiles (indexed 0 to 8),
                                                represented as either 'R' if marked by the red player,
                                                'B' if marked by the blue player, or None if the tile is empty.
            board_winners (List[Optional[str]]): The winners of each local board.
            playable_boards (List[int]): The local boards that may be played in.

        Raises:
            NotImplementedError: This is only an agent base class - please implement a strategy!

        Returns:
            Tuple[int, int]: The local board and tile position to mark.
        """

        raise NotImplementedError()


class UTTTGame:
    def __init__(self, agent: BaseAgent) -> None:
        self.agent = agent
        self.player = None

        self.boards = [[None for _ in range(0, 9)] for _ in range(0, 9)]
        self.board_winners = [None for _ in range(0, 9)]

    def _request_move(self, playable_boards: List[int]) -> Tuple[int, int]:
        """Requests a move from the player's agent.

        Args:
            playable_boards (List[int]): A list of local boards the player's agent can make a move in

        Returns:
            Tuple[int, int]: The local board and tile to mark
        """

        move = self.agent.make_move(
            boards=[board[:] for board in self.boards],  # only give copies lest the
            board_winners=self.board_winners[:],  # lists get modified by the agent
            playable_boards=playable_boards,
        )

        if self.boards[move[0]][move[1]] is not None:
            raise ValueError(
                f"The agent tried to make an illegal move: the tile {move} is already occupied by {self.boards[move[0]][move[1]]}."
            )

        if self.board_winners[move[0]] is not None:
            raise ValueError(
                f"The agent tried to make an illegal move: the tile {move} is in a board already won by {self.boards[move[0]]}."
            )

        return move

    def _place_tile(self, player: str, board: int, tile: int) -> None:
        """Marks a tile in the specified local board for the player specified.

        Args:
            player (str): The player placing the tile (R for red or B for blue)
            board (int): The local board position in the global board
            tile (int): The tile position
        """

        self.boards[board][tile] = player

    def _set_board_winner(self, player: str, board: int) -> None:
        """Marks a local board in the global boad as won for the player specified.

        Args:
            player (str): The winning player (R for red, B for blue or S for stalemate)
            board (int): The local board won
        """

        self.board_winners[board] = player

    def _determine_player(self) -> str:
        """Determines whether the current player is R or B."

        Raises:
            ValueError: An invalid player was received.

        Returns:
            str: The current player
        """

        input_line = input()
        if input_line == "S R":
            return "R"
        elif input_line == "S B":
            return "B"
        else:
            raise ValueError(
                f"The first message should be either `S R` or `S B`, but `{input_line}` was received."
            )

    def play(self):
        """Runs the main game loop."""

        self.player = self._determine_player()
        self.agent.set_player(self.player)

        while True:
            # Get the next line of input and then split it at each space
            parts = input().split(" ")

            # Agent move requested
            if parts[0] == "R":
                playable_boards = [int(board) for board in parts[1].split(",")]

                move = self._request_move(playable_boards)
                self._place_tile(player=self.player, board=move[0], tile=move[1])

                print(f"M {move[0]} {move[1]}")

            # Tile placed
            elif parts[0] == "P":
                self._place_tile(
                    player=parts[1], board=int(parts[2]), tile=int(parts[3])
                )

            # Local board won
            elif parts[0] == "G":
                self._set_board_winner(player=parts[1], board=int(parts[2]))
