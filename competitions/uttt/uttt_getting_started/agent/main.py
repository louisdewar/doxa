import random
from typing import List, Optional, Tuple

from uttt import BaseAgent, UTTTGame

#########################################################################
#                                                                       #
#       Modify the Agent class below to implement your own agent.       #
#       You may define additional methods as you see fit.               #
#                                                                       #
#########################################################################


class Agent(BaseAgent):
    """
    This is an example agent for the ultimate tic-tac-toe competition on Doxa.

    Its current strategy is to play valid moves picked completely at random.
    """
    def make_move(
        self,
        # If you have never seen this syntax before, these are type annotations!
        # Don't worry - they're totally optional
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

            board_winners (List[Optional[str]]): The winners of each local board. While this totally random
                                                 agent does not take local board winners into account, you will
                                                 probably want to in order to implement a better strategy!

            playable_boards (List[int]): The local boards that may be played in.

        Returns:
            Tuple[int, int]: The local board and tile position to mark for your agent.
        """

        ################################################################################
        #                                                                              #
        #                   Replace this section with your own code!                   #
        #                                                                              #
        ################################################################################

        # `self.player` is the player you are currently playing as (either R or B).
        # Likewise, `self.opponent` is your opponent (either B or R).

        # Find all the free tiles across the playable boards.
        possible_moves = [
            (board, tile)
            for board in playable_boards
            for tile in range(0, 9)
            if boards[board][tile] is None
        ]

        # Pick a valid move at random
        move = random.choice(possible_moves)

        ################################################################################

        return move


def main():
    # Instantiate the agent
    agent = Agent()

    # Start playing the game
    game = UTTTGame(agent)
    game.play()


# This is a common Python idiom, which signals
# to other Python programmers that this is a script.
if __name__ == "__main__":
    main()
