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

function SubGrid({ winner, tileOwners, playable }) {
  return (
    <div className={classNames('sub-grid', winnerToClass(winner))}>
      {tileOwners.map((tileWinner, i) => (<div key={i} className={classNames('tile', winnerToClass(tileWinner), { playable: playable && !tileWinner })} />))}
    </div>
  );
}

export default function Grid({ gameState, small = false }) {
  return (
    <div className={classNames('grid', winnerToClass(gameState.winner), { small })}>
      {gameState.subGrids.map((subGrid, i) => {
        return <SubGrid key={i} winner={gameState.subGridsWon[i]} tileOwners={subGrid} playable={gameState.winner === null && (gameState.nextGrid === i || (gameState.nextGrid === null && !gameState.subGridsWon[i]))} />;
      })}
    </div>
  );
}