import api from 'common/api';
import GameState from 'common/gameReducer.js';

import { useState, useEffect, useRef } from 'react';
import { Link } from 'react-router-dom';
import classNames from 'classnames';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faStepForward, faFastForward, faStepBackward, faFastBackward } from '@fortawesome/free-solid-svg-icons';

import './Games.scss';
import Grid from './Grid';

export default function Games({ matchID, winners }) {
  return (
    <div className='games maxwidth'>
      <h2>Showing {winners.length} games</h2>
      {winners.map((winner, i) => {
        return <GameCard key={i} matchID={matchID} gameID={i + 1} winner={winner} />;
      })}
    </div>
  );
}

function GameCard({ matchID, gameID, winner }) {
  const [loaded, setLoaded] = useState(false);
  const [grid, setGrid] = useState(null);
  const [currentMove, setCurrentMove] = useState(0);

  // const [playerInterval, setPlayerInterval] = useState(0);

  const gameState = useRef(new GameState());

  const updateCurrentMove = () => {
    setCurrentMove(gameState.current.getPosition() + 1);
  };

  // Load player and opponent
  useEffect(() => {
    setLoaded(false);
    api.game.getUTTTGameEvents(matchID, gameID).then(events => {
      gameState.current = new GameState();
      gameState.current.addManyEvents(events);
      setGrid(gameState.current.getGrid());
      updateCurrentMove();
      setLoaded(true);
    })
      .catch(err => {
        console.error(err);
      });
  }, [gameID, matchID]);

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

  if (!loaded) {
    return null;
  }

  return (
    <Link to={`/c/uttt/match/${matchID}/game/${gameID}`}>
      <div className={classNames('game-card', { 'won': winner === 'R', 'lost': winner === 'B', 'drawn': winner === 'S' })}>
        <div className="mini-player">
          <Grid gameState={grid} small={true} />
          <div className="controls">
            <FontAwesomeIcon icon={faFastBackward} onClick={goToBeginning} />
            <FontAwesomeIcon icon={faStepBackward} onClick={stepBackward} />
            {/* <FontAwesomeIcon icon={faPlay} /> */}
            <FontAwesomeIcon icon={faStepForward} onClick={stepForward} />
            <FontAwesomeIcon icon={faFastForward} onClick={goToEnd} />
          </div>
          <div className="move-number">{currentMove}/{gameState.current.getLength()}</div>
        </div>
        <div className="labeled-value"><span className="main">&#35;{gameID}</span><span className="label">game</span></div>
      </div>
    </Link>
  );
}
