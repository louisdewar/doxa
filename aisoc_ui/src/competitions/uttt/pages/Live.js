import { faFastBackward, faFastForward, faStepBackward, faStepForward } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import classNames from 'classnames';
import Grid from 'competitions/uttt/components/Grid';
import LiveSocketManager from 'competitions/uttt/services/liveSocketManager';
import React, { useEffect, useRef, useState } from 'react';
import { useParams } from 'react-router';
import Layout from '../components/Layout';
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

export default function Live({ competitionBaseUrl }) {
  const { agentID } = useParams();
  const [loaded, setLoaded] = useState(false);
  const [grid, setGrid] = useState(null);
  const [moves, setMoves] = useState(null);

  const [currentMove, setCurrentMove] = useState(0);

  // const [playerInterval, setPlayerInterval] = useState(0);

  const socketManager = useRef(new LiveSocketManager());

  const updateCurrentMove = () => {
    setCurrentMove(socketManager.current.gameState.getPosition() + 1);
  };

  useEffect(async () => {
    socketManager.current = new LiveSocketManager();
    setGrid(socketManager.current.gameState.getGrid());
    updateCurrentMove();
    setMoves(socketManager.current.gameState.toMoveList());

    setLoaded(true);
  }, []);

  // const stepForward = e => {
  //   e.preventDefault();
  //   gameState.current.next();
  //   updateCurrentMove();
  //   setGrid(gameState.current.getGrid());
  // };

  // const stepBackward = e => {
  //   e.preventDefault();
  //   gameState.current.previous();
  //   updateCurrentMove();
  //   setGrid(gameState.current.getGrid());
  // };

  // const goToBeginning = e => {
  //   e.preventDefault();
  //   gameState.current.goToBeginning();
  //   updateCurrentMove();
  //   setGrid(gameState.current.getGrid());
  // };

  // const goToEnd = e => {
  //   e.preventDefault();
  //   gameState.current.goToEnd();
  //   updateCurrentMove();
  //   setGrid(gameState.current.getGrid());
  // };

  // const goToMove = (pos, e) => {
  //   e.preventDefault();
  //   gameState.current.goToPosition(pos);
  //   updateCurrentMove();
  //   setGrid(gameState.current.getGrid());
  // };


  const stepForward = null;
  const stepBackward = null;
  const goToBeginning = null;
  const goToEnd = null;
  const goToMove = null;

  if (!loaded) {
    return null;
  }

  return (
    <Layout competitionBaseUrl={competitionBaseUrl}>
      <div className="game-header">
        <div className="player-versus">
          <span className="player">Play against agent: {agentID}</span>
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
    </Layout>
  );
}
