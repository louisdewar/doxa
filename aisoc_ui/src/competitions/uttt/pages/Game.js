import { faFastBackward, faFastForward, faStepBackward, faStepForward } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import classNames from 'classnames';
import Grid from 'competitions/uttt/components/Grid';
import GameState from 'competitions/uttt/services/gameReducer';
import Navbar from 'components/NavBar';
import React, { useEffect, useRef, useState } from 'react';
import { useParams } from 'react-router';
import { Link } from 'react-router-dom';
import UTTTAPI from '../api';
import './Game.scss';


function moveToLabel(move) {
  return `${move.g} ${move.t}`;
}

function Moves({ moves, currentMove, goToMove }) {
  return (
    <div className="moves">
      {moves.map((move, i) => {
        let output;
        if (i % 2 == 0) {
          output = <span className="number">{Math.floor(i / 2) + 1}</span>;
        }
        return <React.Fragment key={i}>{output}<span className={classNames('move', { current: i === currentMove - 1, next: i === currentMove })} onClick={goToMove.bind(null, i)}>{moveToLabel(move)}</span></React.Fragment>;
      })}
    </div>
  );
}

export default function Game() {
  const api = new UTTTAPI();

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

    setPlayers(await api.getGamePlayers(matchID));
    const events = await api.getUTTTGameEvents(matchID, gameID);

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

  return (
    <>
      <Navbar competitionName='Ultimate Tic-Tac-Toe' homepageUrl='/c/uttt/' />
      <div className="maxwidth">
        <div className="game-header">
          <div className="player-versus">
            <Link to={`/c/uttt/user/${players[0].username}`}><span className="player player-1">{players[0].username}</span></Link>
            <span className="separator">VS</span>
            <Link to={`/c/uttt/user/${players[1].username}`}><span className="player player-2">{players[1].username}</span></Link>
          </div>
        </div>
        <div className="game-wrapper">
          <div className="game-grid">
            <Grid gameState={grid} />
          </div>
          <div className="move-list">
            <h2>Move List</h2>
            <Moves moves={moves} currentMove={currentMove} goToMove={goToMove} />
            <div className="controls">
              <div className="move-button" onClick={goToBeginning}>
                <FontAwesomeIcon icon={faFastBackward} />
              </div>
              <div className="move-button" onClick={stepBackward}>
                <FontAwesomeIcon icon={faStepBackward} />
              </div>
              {/* <FontAwesomeIcon icon={faPlay} /> */}
              <div className="move-button" onClick={stepForward}>
                <FontAwesomeIcon icon={faStepForward} />
              </div>
              <div className="move-button" onClick={goToEnd}>
                <FontAwesomeIcon icon={faFastForward} />
              </div>
            </div>
          </div>
        </div>
      </div>

    </>
  );
}
