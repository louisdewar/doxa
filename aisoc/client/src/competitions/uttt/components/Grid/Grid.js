import classNames from 'classnames';
import './Grid.scss';

function winnerToClass(winner) {
  if (!winner) {
    return null;
  } if (winner === 'R') {
    return 'player-a';
  } else if (winner === 'B') {
    return 'player-b';
  } else if (winner === 'S') {
    return 'stalemate';
  } else {
    throw new Error('Unknown winner type: ' + winner);
  }
}

function SubGrid({ winner, tileOwners, playable, grid, onTileClick }) {
  return (
    <div className={classNames('sub-grid', winnerToClass(winner))}>
      {tileOwners.map((tileWinner, i) => (<div
        key={i}
        className={classNames('tile', winnerToClass(tileWinner), { playable: playable && !tileWinner })}
        onClick={onTileClick? () => { onTileClick(grid, i); }: undefined}
      />))}
    </div>
  );
}

export default function Grid({ gameState, small = false, onTileClick }) {
  if (!gameState) {
    // TODO: Maybe just render a blank grid to avoid jumpy movement on load
    return null;
  }

  return (
    <div className={classNames('grid', winnerToClass(gameState.winner), { small, playable: !!onTileClick })}>
      {gameState.subGrids.map((subGrid, i) => {
        return <SubGrid
          key={i}
          grid={i}
          winner={gameState.subGridsWon[i]}
          tileOwners={subGrid}
          playable={gameState.winner === null && (gameState.nextGrid === i || (gameState.nextGrid === null && !gameState.subGridsWon[i]))}
          onTileClick={onTileClick}
        />;
      })}
    </div>
  );
}
