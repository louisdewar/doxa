import { faFastBackward, faFastForward, faStepBackward, faStepForward } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import classNames from 'classnames';
import { useEffect, useRef, useState } from 'react';
import { Link } from 'react-router-dom';
import UTTTAPI from '../api';
import GameState from '../services/gameReducer';
import './Games.scss';
import Grid from './Grid';



export default function Games({ matchID, winners, competitionBaseUrl }) {
  return <>
    <h3 className="games-showing-n-label">Showing {winners.length} games</h3>
    <div className='games'>
      {winners.map((winner, i) => {
        return <GameCard key={i} matchID={matchID} gameID={i + 1} winner={winner} competitionBaseUrl={competitionBaseUrl} />;
      })}
    </div>
  </>;
}

function GameCard({ matchID, gameID, winner, competitionBaseUrl }) {
  const [loaded, setLoaded] = useState(false);
  const [grid, setGrid] = useState(null);
  const [currentMove, setCurrentMove] = useState(0);

  const gameState = useRef(new GameState());

  const updateCurrentMove = () => {
    setCurrentMove(gameState.current.getPosition() + 1);
  };

  // Load player and opponent
  useEffect(() => {
    setLoaded(false);
    UTTTAPI.getUTTTGameEvents(matchID, gameID).then(events => {
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
        <div className="labeled-value">
          <span className="label">Game</span>
          <span className="main">&#35;{gameID}</span>
        </div>
        <div className="mini-player">

          <div className="move-number">
            {currentMove}/{gameState.current.getLength()}
            <div className="controls">
              <a onClick={goToBeginning}><FontAwesomeIcon icon={faFastBackward} fixedWidth /></a>
              <a onClick={stepBackward}><FontAwesomeIcon icon={faStepBackward} fixedWidth /></a>
              <a onClick={stepForward}><FontAwesomeIcon icon={faStepForward} fixedWidth /></a>
              <a onClick={goToEnd}><FontAwesomeIcon icon={faFastForward} fixedWidth /></a>
            </div>
          </div>
          <Grid gameState={grid} small={true} />
        </div>
      </div>
    </Link>
  );
}
