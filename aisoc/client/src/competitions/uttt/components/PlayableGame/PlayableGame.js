import { useEffect, useRef, useState } from 'react';
import Grid from '../Grid/Grid';
import PlayableGameController from './PlayableGameController';


export default function PlayableGame({ resets, agent }) {
  const controller = useRef(new PlayableGameController(agent));
  const [grid, setGrid] = useState(() => {
    return controller.current.getGrid();
  });


  useEffect(() => {
    if (resets == 0) return;

    setGrid(controller.current.reset());
  }, [resets]);


  return <Grid
    gameState={grid}
    onTileClick={(g, t) => {
      for (const updatedGrid of controller.current.placeTile(g, t)) {
        setGrid(updatedGrid);
      }
    }}
  />;
}
