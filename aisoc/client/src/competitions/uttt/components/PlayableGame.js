import { useRef, useState } from 'react';
import GameState from '../services/gameReducer';
import Grid from './Grid';


function getPlayableTiles(state) {
  if (state.winner !== null) return [];

  if (state.nextGrid !== null) { // && !state.subGridsWon[state.nextGrid] (depending on the game version)
    return [...Array(9).keys()]
      .filter(tile => state.subGrids[state.nextGrid][tile] === null)
      .map(tile => ({ g: state.nextGrid, t: tile }));
  }

  const playableTiles = [];
  for (let grid = 0; grid < 9; ++grid) {
    if (!state.subGridsWon[grid]) {
      playableTiles.push(...[...Array(9).keys()]
        .filter(tile => state.subGrids[grid][tile] === null)
        .map(tile => ({ g: grid, t: tile })));
    }
  }

  return playableTiles;
}


function isGridWon(subGrid) {
  return {
    // Rows
    [[subGrid[0], subGrid[1], subGrid[2]].every((e, i, a) => a[0] && e == a[0])]: subGrid[0],
    [[subGrid[3], subGrid[4], subGrid[5]].every((e, i, a) => a[0] && e == a[0])]: subGrid[3],
    [[subGrid[6], subGrid[7], subGrid[8]].every((e, i, a) => a[0] && e == a[0])]: subGrid[6],

    // Columns
    [[subGrid[0], subGrid[3], subGrid[6]].every((e, i, a) => a[0] && e == a[0])]: subGrid[0],
    [[subGrid[1], subGrid[4], subGrid[7]].every((e, i, a) => a[0] && e == a[0])]: subGrid[1],
    [[subGrid[2], subGrid[5], subGrid[8]].every((e, i, a) => a[0] && e == a[0])]: subGrid[2],

    // Diagonals
    [[subGrid[0], subGrid[4], subGrid[8]].every((e, i, a) => a[0] && e == a[0])]: subGrid[0],
    [[subGrid[2], subGrid[4], subGrid[6]].every((e, i, a) => a[0] && e == a[0])]: subGrid[2],
  }[true];
}

class RandomAgent {
  constructor(player) {
    this.player = player;
    this.opponent = player == 'R' ? 'B' : 'R';
  }

  playMove(state) {
    const playableTiles = getPlayableTiles(state);
    return playableTiles[Math.floor(Math.random() * playableTiles.length)];
  }
}

export default function PlayableGame() {
  const gameState = useRef(new GameState());
  const randomAgent = useRef(new RandomAgent('R'));

  const [grid, setGrid] = useState(() => {
    gameState.current.addEvent(randomAgent.current.playMove(gameState.current.state));
    return gameState.current.getGrid();
  });

  const checkForGridWin = () => {
    for (const i in gameState.current.state.subGrids) {
      const w = isGridWon(gameState.current.state.subGrids[i]);
      if (w !== undefined && gameState.current.state.subGrids[i] != null) {
        gameState.current.addEvent({ g: i, w });

        const overall = isGridWon(gameState.current.state.subGridsWon);
        if (overall) {
          gameState.current.addEvent({ overall });
          return true;
        }
      }
    }
  };


  return <>
    <Grid
      gameState={grid}
      onTileClick={(grid, tile) => {
        if (gameState.current.state.winner) return;

        const playableTiles = getPlayableTiles(gameState.current.state);

        if (playableTiles.find(x => x.g == grid && x.t == tile)) {
          gameState.current.addEvent({ g: grid, t: tile });
          const w = checkForGridWin();
          setGrid(gameState.current.getGrid());

          if (w) return;

          const move = randomAgent.current.playMove(gameState.current.state);
          gameState.current.addEvent(move);
          checkForGridWin();
          setGrid(gameState.current.getGrid());
        }
      }}
    />
  </>;
}
