import { faFastBackward, faFastForward, faStepBackward, faStepForward } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import classNames from 'classnames';
import { useEffect, useRef, useState } from 'react';
import { Link } from 'react-router-dom';
import GameState from '../services/gameReducer';
import './Games.scss';
import Grid from './Grid';


export default function Games({ matchID, games, competitionBaseUrl, extra = null }) {
  return <>
    <h3 className="games-showing-n-label">Showing {games.length} games</h3>
    <div className='games'>
      {games.map((game, i) => {
        return <GameCard key={i} matchID={matchID} gameID={i + 1} game={game} competitionBaseUrl={competitionBaseUrl} />;
      })}
      {extra}
    </div>
  </>;
}

function GameCard({ game, competitionBaseUrl, gameID, matchID }) {
  const [grid, setGrid] = useState(null);
  const [currentMove, setCurrentMove] = useState(0);

  const gameState = useRef(new GameState());

  const winner = game.overall_winner;

  const updateCurrentMove = () => {
    setCurrentMove(gameState.current.getPosition() + 1);
  };

  useEffect(() => {
    gameState.current = new GameState();
    gameState.current.addManyEvents(game.events);
    setGrid(gameState.current.getGrid());
    updateCurrentMove();
  }, [game]);

  const stepForward = e => {
    e.preventDefault();
    gameState.current.next();
    updateCurrentMove();
    setGrid(gameState.current.getGrid());
  };

  const stepBackward = e => {
    e.preventDefault();
    gameState.current.previous();
    updateCurrentMove();
    setGrid(gameState.current.getGrid());
  };

  const goToBeginning = e => {
    e.preventDefault();
    gameState.current.goToBeginning();
    updateCurrentMove();
    setGrid(gameState.current.getGrid());
  };

  const goToEnd = e => {
    e.preventDefault();
    gameState.current.goToEnd();
    updateCurrentMove();
    setGrid(gameState.current.getGrid());
  };

  return (
    <Link to={`${competitionBaseUrl}match/${matchID}/game/${gameID}`}>
      <div className={classNames('game-card', { 'won': winner === 'R', 'lost': winner === 'B', 'drawn': winner === 'S' })}>
        <div className="labeled-value">
          <span className="label">Game</span>
          <span className="main">&#35;{gameID}</span>
        </div>
        <div className="mini-player">
          <div className="move-number">
            {currentMove}/{gameState.current.getLength()}
            <div className="controls">
              <button onClick={goToBeginning}><FontAwesomeIcon icon={faFastBackward} fixedWidth /></button>
              <button onClick={stepBackward}><FontAwesomeIcon icon={faStepBackward} fixedWidth /></button>
              <button onClick={stepForward}><FontAwesomeIcon icon={faStepForward} fixedWidth /></button>
              <button onClick={goToEnd}><FontAwesomeIcon icon={faFastForward} fixedWidth /></button>
            </div>
          </div>
          <Grid gameState={grid} small={true} />
        </div>
      </div>
    </Link>
  );
}
