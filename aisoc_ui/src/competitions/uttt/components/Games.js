import { faFastBackward, faFastForward, faStepBackward, faStepForward } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import classNames from 'classnames';
import GameState from 'competitions/uttt/services/gameReducer';
import { useEffect, useRef, useState } from 'react';
import { Link } from 'react-router-dom';
import UTTTAPI from '../api';
import './Games.scss';
import Grid from './Grid';



export default function Games({ matchID, winners, competitionBaseUrl }) {
  return (
    <div className='games maxwidth'>
      <h2>Showing {winners.length} games</h2>
      {winners.map((winner, i) => {
        return <GameCard key={i} matchID={matchID} gameID={i + 1} winner={winner} competitionBaseUrl={competitionBaseUrl} />;
      })}
    </div>
  );
}

function GameCard({ matchID, gameID, winner, competitionBaseUrl }) {
  const api = new UTTTAPI();

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
    api.getUTTTGameEvents(matchID, gameID).then(events => {
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
    <Link to={`${competitionBaseUrl}match/${matchID}/game/${gameID}`}>
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
