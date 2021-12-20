import Card from 'components/Card';
import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import { useAuth } from 'hooks/useAuth';
import UTTTAPI from '../api';
import Games from '../components/Games';
import './Match.scss';

import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faExclamationTriangle } from '@fortawesome/free-solid-svg-icons';
import PlayerLink from '../components/PlayerLink';

const PLAYER_CLASS = ['main', 'opposing'];

async function loadMatchData(matchID, authToken) {
  const winners = await UTTTAPI.getUTTTGameWinners(matchID) || [];
  const scores = await UTTTAPI.getUTTTGameScores(matchID);
  const players = await UTTTAPI.getGamePlayers(matchID);

  if (!players) {
    return null;
  }

  const error = await UTTTAPI.getSingleGameEvent(matchID, '_ERROR', authToken);

  if (!scores) {
    return { winners, players, error };
  }

  const total = scores.a_wins + scores.b_wins + scores.draws;

  const forfeit = await UTTTAPI.getSingleGameEvent(matchID, '_FORFEIT', authToken);
  if (forfeit) {
    forfeit.payload.remaining = total - winners.length;
  }

  const calculatePercentage = number => 100 * number / total;
  scores.percentages = {
    a_wins: calculatePercentage(scores.a_wins),
    b_wins: calculatePercentage(scores.b_wins),
    draws: calculatePercentage(scores.draws)
  };

  return { winners, scores, players, forfeit, error };
}


function ErrorCard({ forfeit, error, players,  baseUrl }) {

  let errorMessage;

  let extraInfo;

  if (forfeit) {
    const forfeiter = forfeit.payload.agent;
    const other = forfeiter === 0? 1: 0;
    const stderr = forfeit.payload.stderr;
    const remaining = forfeit.payload.remaining;

    errorMessage = (
      <>
        <p><PlayerLink username={players[forfeiter].username} baseUrl={baseUrl} playerClass={PLAYER_CLASS[forfeiter]} />&apos;s agent forfeit the match!</p>
        <p>
        This means that <PlayerLink username={players[other].username} baseUrl={baseUrl} playerClass={PLAYER_CLASS[other]} /> wins
        the remaining {remaining} {remaining > 1? 'games': 'game'} by default.
        </p>
      </>
    );

    if (stderr) {
      extraInfo = (
        <>
          <p className="logs-message">You have permission to view the <code>stderr</code> output of <PlayerLink username={players[forfeiter].username} baseUrl={baseUrl} playerClass={PLAYER_CLASS[forfeiter]} />&apos;s agent (max 50mb):</p>
          <pre className="logs">{stderr}</pre>
        </>
      );
    }
  }

  if (error) {
    // If the error was not a forfeit it represents an internal error
    if (!forfeit) {
      errorMessage = (
        <>
          <p>An internal error occured when running this match that meant it couldn&apos;t continue.</p>
          <p>
            The match can be re-rescheduled by either <PlayerLink username={players[0].username} baseUrl={baseUrl} playerClass={'main'} /> {' '}
            or <PlayerLink username={players[1].username} baseUrl={baseUrl} playerClass={'opposing'} /> re-uploading their agent.
          </p>
        </>
      );
    }

    if (error.payload.vm_logs) {
      const vm_logs = error.payload.vm_logs;
      extraInfo = <>
        {extraInfo}
        <p className="logs-message">VM logs for <PlayerLink username={players[0].username} baseUrl={baseUrl} playerClass={'main'} /></p>
        <pre className="logs">
          {vm_logs[0]}
        </pre>

        <p className="logs-message">VM logs for <PlayerLink username={players[1].username} baseUrl={baseUrl} playerClass={'opposing'} /></p>
        <pre className="logs">
          {vm_logs[1]}
        </pre>
      </>;
    }
  }

  return (
    <div className="game-card error">
      <div className="error-icon"><FontAwesomeIcon icon={faExclamationTriangle} /></div>
      {errorMessage}
      {extraInfo}
    </div>
  );
}

function TitleCard({ players, scores, baseUrl }) {

  let scoresSection;
  if (scores) {
    scoresSection = <>
      <h2>
        {scores.a_wins} wins | {scores.draws} draws | {scores.b_wins} losses
      </h2>
      <div className='match-score-bar'>
        {scores.percentages.a_wins > 0 && <div className='match-score-bar-wins' style={{ width: scores.percentages.a_wins + '%' }}></div>}
        {scores.percentages.draws > 0 && <div className='match-score-bar-draws' style={{ width: scores.percentages.draws + '%' }}></div>}
        {scores.percentages.b_wins > 0 && <div className='match-score-bar-losses' style={{ width: scores.percentages.b_wins + '%' }}></div>}
      </div>
    </>;
  } else {
    scoresSection = <h2>No scores</h2>;
  }

  return <Card darker className="match-page-header">
    <h1><PlayerLink username={players[0].username} baseUrl={baseUrl} playerClass={'main'} /> vs <PlayerLink username={players[1].username} baseUrl={baseUrl} playerClass={'opposing'} />
    </h1>
    {scoresSection}
  </Card>;
}

export default function Match({ baseUrl }) {
  const { id } = useParams();
  const [data, setData] = useState(null);
  const auth = useAuth();

  useEffect(async () => {
    setData(await loadMatchData(id, auth.token));
  }, []);

  if (!data) {
    return <></>;
  }

  let extraCards = null;

  if (data.forfeit || data.error) {
    extraCards = <ErrorCard error={data.error} forfeit={data.forfeit} players={data.players} baseUrl={baseUrl} />;
    //const { agent, remaining, stderr } = data.forfeit.payload;
    //extraCards = <ErrorCard players={data.players} forfeiter={agent} remaining={remaining} stderr={stderr} baseUrl={baseUrl} />;
  }

  return <>
    <span></span><span></span><span></span><span></span> {/* a fun hack just to get a better outline colour below! */}
    <TitleCard players={data.players} scores={data.scores} baseUrl={baseUrl} />

    <Games matchID={id} winners={data.winners} competitionBaseUrl={baseUrl} extra={extraCards} />
  </>;
}
