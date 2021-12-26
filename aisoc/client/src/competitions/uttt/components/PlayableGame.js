import { useRef, useState } from 'react';
import GameState from '../services/gameReducer';
import Grid from './Grid';


function getPlayableTiles(state) {
  if (state.winner !== null) return [];

  return (state.nextGrid != null ? [state.nextGrid] : [...Array(9).keys()].filter(grid => !state.subGridsWon[grid]))
    .flatMap(grid => [...Array(9).keys()]
      .filter(tile => state.subGrids[grid][tile] === null)
      .map(tile => ({ g: grid, t: tile })));
}


function isBoardWon(subGrid) {
  return {
    // Rows
    [[subGrid[0], subGrid[1], subGrid[2]].every((e, _, a) => a[0] && e == a[0])]: subGrid[0],
    [[subGrid[3], subGrid[4], subGrid[5]].every((e, _, a) => a[0] && e == a[0])]: subGrid[3],
    [[subGrid[6], subGrid[7], subGrid[8]].every((e, _, a) => a[0] && e == a[0])]: subGrid[6],

    // Columns
    [[subGrid[0], subGrid[3], subGrid[6]].every((e, _, a) => a[0] && e == a[0])]: subGrid[0],
    [[subGrid[1], subGrid[4], subGrid[7]].every((e, _, a) => a[0] && e == a[0])]: subGrid[1],
    [[subGrid[2], subGrid[5], subGrid[8]].every((e, _, a) => a[0] && e == a[0])]: subGrid[2],

    // Diagonals
    [[subGrid[0], subGrid[4], subGrid[8]].every((e, _, a) => a[0] && e == a[0])]: subGrid[0],
    [[subGrid[2], subGrid[4], subGrid[6]].every((e, _, a) => a[0] && e == a[0])]: subGrid[2],
  }[true];
}

function getRandomMove(playableTiles) {
  return playableTiles[Math.floor(Math.random() * playableTiles.length)];
}

export default function PlayableGame() {
  const gameState = useRef(new GameState());

  const [grid, setGrid] = useState(() => {
    gameState.current.addEvent(getRandomMove(getPlayableTiles(gameState.current.state)));
    return gameState.current.getGrid();
  });

  const findWins = () => {
    for (const i in gameState.current.state.subGrids) {
      const w = isBoardWon(gameState.current.state.subGrids[i]);
      if (w !== undefined && gameState.current.state.subGrids[i] != null) {
        gameState.current.addEvent({ g: i, w });

        const overall = isBoardWon(gameState.current.state.subGridsWon);
        if (overall) {
          gameState.current.addEvent({ overall });
          return true;
        }
      }
    }
    return false;
  };

  return <Grid
    gameState={grid}
    onTileClick={(grid, tile) => {
      if (gameState.current.state.winner) return;

      const playableTiles = getPlayableTiles(gameState.current.state);
      if (playableTiles.find(x => x.g == grid && x.t == tile)) {
        gameState.current.addEvent({ g: grid, t: tile });
        setGrid(gameState.current.getGrid());

        if (!findWins()) {
          gameState.current.addEvent(getRandomMove(getPlayableTiles(gameState.current.state)));
          findWins();
        }

        setGrid(gameState.current.getGrid());
      }
    }}
  />;
}
