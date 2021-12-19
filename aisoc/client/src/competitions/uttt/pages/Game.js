import { faFastBackward, faFastForward, faSquare, faStepBackward, faStepForward } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import classNames from 'classnames';
import Card from 'components/Card';
import React, { useEffect, useRef, useState } from 'react';
import { useParams } from 'react-router';
import { Link } from 'react-router-dom';
import UTTTAPI from '../api';
import Grid from '../components/Grid';
import GameState from '../services/gameReducer';
import './Game.scss';


function moveToLabel(move) {
  return `[${move.g}, ${move.t}]`;
}

function Moves({ moves, currentMove, goToMove }) {
  return (
    <div className="game-move-list">
      <span className="number">#</span>
      <span className="game-move-header-item">[<FontAwesomeIcon icon={faSquare} size='lg' />, <FontAwesomeIcon icon={faSquare} size='xs' />]</span>
      <span className="game-move-header-item">[<FontAwesomeIcon icon={faSquare} size='lg' />, <FontAwesomeIcon icon={faSquare} size='xs' />]</span>
      {moves.map((move, i) => {
        let output;
        if (i % 2 == 0) {
          output = <span className="number">{Math.floor(i / 2) + 1}</span>;
        }
        return <React.Fragment key={i}>
          {output}<span
            className={classNames('move', { current: i === currentMove - 1, next: i === currentMove })}
            onClick={goToMove.bind(null, i)}
          >
            {moveToLabel(move)}
          </span>
        </React.Fragment>;
      })}
    </div>
  );
}

export default function Game({ baseUrl }) {
  const { matchID, gameID } = useParams();
  const [players, setPlayers] = useState(null);
  const [loaded, setLoaded] = useState(false);
  const [grid, setGrid] = useState(null);
  const [moves, setMoves] = useState(null);

  const [currentMove, setCurrentMove] = useState(0);

  const gameState = useRef(new GameState());

  const updateCurrentMove = () => {
    setCurrentMove(gameState.current.getPosition() + 1);
  };

  useEffect(async () => {

    setPlayers(await UTTTAPI.getGamePlayers(matchID));
    const events = await UTTTAPI.getUTTTGameEvents(matchID, gameID);

    gameState.current = new GameState();
    gameState.current.addManyEvents(events);
    setGrid(gameState.current.getGrid());
    updateCurrentMove();
    setMoves(gameState.current.toMoveList());

    setLoaded(true);
  }, [matchID, gameID]);

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

  const goToMove = (pos, e) => {
    e.preventDefault();
    gameState.current.goToPosition(pos);
    updateCurrentMove();
    setGrid(gameState.current.getGrid());
  };

  if (!loaded) {
    return null;
  }

  return <>
    <span></span><span></span><span></span><span></span> {/* a fun hack just to get a better outline colour below! */}

    <Card darker className="game-page-header">
      <h1>
        <Link to={`${baseUrl}user/${players[0].username}`} className="game-page-main-player-link">{players[0].username}</Link> vs <Link to={`${baseUrl}user/${players[1].username}`} className="game-page-opposing-player-link">{players[1].username}</Link>
      </h1>
      <h2>
        Game #{gameID}
      </h2>
    </Card>

    <div className="game-container">
      <div className="game-grid">
        <Grid gameState={grid} />
      </div>
      <div className="game-moves">
        <h3>Game moves</h3>
        <Moves moves={moves} currentMove={currentMove} goToMove={goToMove} />
        <div className="controls">
          <div className="move-button" onClick={goToBeginning}>
            <FontAwesomeIcon icon={faFastBackward} fixedWidth />
          </div>
          <div className="move-button" onClick={stepBackward}>
            <FontAwesomeIcon icon={faStepBackward} fixedWidth />
          </div>
          <div className="move-button" onClick={stepForward}>
            <FontAwesomeIcon icon={faStepForward} fixedWidth />
          </div>
          <div className="move-button" onClick={goToEnd}>
            <FontAwesomeIcon icon={faFastForward} fixedWidth />
          </div>
        </div>
      </div>
    </div>
  </>;
}
