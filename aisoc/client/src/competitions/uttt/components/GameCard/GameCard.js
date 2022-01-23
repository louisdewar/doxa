import { faExclamationTriangle, faFastBackward, faFastForward, faStepBackward, faStepForward } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import classNames from 'classnames';
import { useEffect, useRef, useState } from 'react';
import { Link } from 'react-router-dom';
import GameState from '../Grid/GameState';
import Grid from '../Grid/Grid';
import PlayerLink from '../PlayerLink/PlayerLink';
import './GameCard.scss';

const PLAYER_CLASS = ['main', 'opposing'];

export default function GameCard({ game, baseUrl, gameID, matchID, players, forfeit }) {
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
    <Link to={`${baseUrl}match/${matchID}/game/${gameID}`}>
      <div className='game-card'>
        <div className={classNames('game-card-body', { 'won': winner === 'R', 'lost': winner === 'B', 'drawn': winner === 'S', 'forfeited': game.forfeit })}>
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
        {game.forfeit && <ForfeitWarning baseUrl={baseUrl} players={players} forfeit={forfeit} moveCount={gameState.current.getLength()} />}
      </div>
    </Link>
  );
}

function ForfeitWarning({ baseUrl, players, forfeit, moveCount }) {
  const forfeiter = forfeit.payload.agent;
  const other = forfeiter === 0 ? 1 : 0;
  const remaining = forfeit.payload.remaining ?? 0;

  return <div className='game-card-forfeit-warning'>
    <FontAwesomeIcon icon={faExclamationTriangle} fixedWidth />
    <span>
      <PlayerLink username={players[forfeiter].username} baseUrl={baseUrl} playerClass={PLAYER_CLASS[forfeiter]} />&apos;s agent forfeited the game after {moveCount} moves, so <PlayerLink username={players[other].username} baseUrl={baseUrl} playerClass={PLAYER_CLASS[other]} /> wins this {remaining == 0 ? 'game' : `and the remaining ${remaining} ${remaining > 1 ? 'games' : 'game'} in the match`} by default.
    </span>
  </div>;
}
