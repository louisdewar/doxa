import { faClock, faExclamationTriangle, faHourglassEnd } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { DoxaError } from 'api/common';
import Card from 'components/Card';
import { useAuth } from 'hooks/useAuth';
import Error404 from 'pages/Error404';
import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import { formatDuration, formatTime } from 'utils/time';
import UTTTAPI from '../api';
import Games from '../components/Games';
import PlayerLink from '../components/PlayerLink';
import './Match.scss';


const PLAYER_CLASS = ['main', 'opposing'];

async function loadMatchData(matchID, authToken) {
  const game = await UTTTAPI.getGame(matchID, null);
  const players = await UTTTAPI.getGamePlayers(matchID);
  const events = await UTTTAPI.getGameEvents(matchID, null, authToken);

  const games = [];

  let error, forfeit, scores;

  for (let event of events) {
    if (event.type.startsWith('game_') && event.type != 'game_winners') {
      // NOTE: if we want to include the timestamp of the game in the UI we can copy game.timestamp
      // into game.payload
      games.push(event.payload);
    } else if (event.type === '_ERROR') {
      error = event;
    } else if (event.type === '_FORFEIT') {
      forfeit = event;
    } else if (event.type === 'scores') {
      scores = event.payload;
    }
  }

  if (scores) {
    const total = scores.a_wins + scores.b_wins + scores.draws;
    const calculatePercentage = number => 100 * number / total;
    scores.percentages = {
      a_wins: calculatePercentage(scores.a_wins),
      b_wins: calculatePercentage(scores.b_wins),
      draws: calculatePercentage(scores.draws)
    };

    if (forfeit && forfeit.payload) {
      forfeit.payload.remaining = total - games.length;
    }
  }

  return { games, queuedAt: game.queued_at, startedAt: game.started_at, completedAt: game.completed_at, error, forfeit, players, scores };
}


function ErrorCard({ forfeit, error, players, baseUrl }) {
  let errorMessage;
  let extraInfo;

  if (forfeit && forfeit.payload) {
    const forfeiter = forfeit.payload.agent;
    const other = forfeiter === 0 ? 1 : 0;
    const stderr = forfeit.payload.stderr;
    const remaining = forfeit.payload.remaining ?? 0;

    errorMessage = (
      <>
        <p><PlayerLink username={players[forfeiter].username} baseUrl={baseUrl} playerClass={PLAYER_CLASS[forfeiter]} />&apos;s agent forfeit the match!</p>
        <p>
          This means that <PlayerLink username={players[other].username} baseUrl={baseUrl} playerClass={PLAYER_CLASS[other]} /> wins
          the remaining {remaining} {remaining > 1 ? 'games' : 'game'} by default.
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

    if (error.payload) {
      if (error.payload.error) {
        extraInfo = <>
          {extraInfo}
          <p className="logs-message">Error message:</p>
          <pre className="logs">
            {error.payload.error}
          </pre>
        </>;
      }

      if (error.payload.debug) {
        extraInfo = <>
          {extraInfo}
          <p className="logs-message">Debug error message:</p>
          <pre className="logs">
            {error.payload.debug}
          </pre>
        </>;
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
  }

  return (
    <div className="game-card error">
      <div className="large-icon error"><FontAwesomeIcon icon={faExclamationTriangle} /></div>
      {errorMessage}
      {extraInfo}
    </div>
  );
}

function OngoingCard() {
  return (
    <div className="game-card ongoing">
      <div className="large-icon ongoing"><FontAwesomeIcon icon={faClock} /></div>
      <p>This match is ongoing — there may be more events in the future.</p>
    </div>
  );
}

function TitleCard({ players, scores, baseUrl, completedAt, queuedAt, startedAt }) {
  const duration = completedAt ? formatDuration((completedAt.getTime() - startedAt.getTime()) / 1000) : null;
  const end = completedAt ? `This match finished ${formatTime(completedAt)}.` :
    (startedAt ? `This match started ${formatTime(startedAt)}.` : `This match was queued ${formatTime(queuedAt)}.`);

  const scoresSection = scores ? <>
    <h2>
      {scores.a_wins} wins | {scores.draws} draws | {scores.b_wins} losses
    </h2>
    <div className='match-score-bar'>
      {scores.percentages.a_wins > 0 && <div className='match-score-bar-wins' style={{ width: scores.percentages.a_wins + '%' }}></div>}
      {scores.percentages.draws > 0 && <div className='match-score-bar-draws' style={{ width: scores.percentages.draws + '%' }}></div>}
      {scores.percentages.b_wins > 0 && <div className='match-score-bar-losses' style={{ width: scores.percentages.b_wins + '%' }}></div>}
    </div>
  </> : <h2>No scores</h2>;

  return <Card darker className="match-page-header">
    <h1><PlayerLink username={players[0].username} baseUrl={baseUrl} playerClass={'main'} /> vs <PlayerLink username={players[1].username} baseUrl={baseUrl} playerClass={'opposing'} />
    </h1>
    {scoresSection}
    <p className="completed">
      <FontAwesomeIcon icon={faClock} size="sm" fixedWidth /> {end}<br />
      {duration ? <><FontAwesomeIcon icon={faHourglassEnd} size="sm" fixedWidth /> This match took {duration} to complete.</> : null}
    </p>
  </Card>;
}

export default function Match({ baseUrl }) {
  const { id } = useParams();
  const [data, setData] = useState(null);
  const auth = useAuth();

  const [notFound, setNotFound] = useState(false);

  useEffect(async () => {
    try {
      setData(await loadMatchData(id, auth.token));
    } catch (e) {
      console.error(e);
      if (e instanceof DoxaError) {
        if (e.status_code === 404) {
          setNotFound(true);
        }
        // TODO: create generic error card
      }
    }
  }, []);


  if (notFound) {
    return <Error404 />;
  }

  if (!data) {
    return <></>;
  }

  let extraCards = null;

  if (data.forfeit || data.error) {
    extraCards = <ErrorCard error={data.error} forfeit={data.forfeit} players={data.players} baseUrl={baseUrl} />;
  }

  if (!data.completedAt) {
    extraCards = <>{extraCards}<OngoingCard /></>;
  }

  return <>
    <span></span><span></span><span></span><span></span> {/* a fun hack just to get a better outline colour below! */}
    <TitleCard players={data.players} scores={data.scores} completedAt={data.completedAt} queuedAt={data.queuedAt} startedAt={data.startedAt} baseUrl={baseUrl} />

    <Games matchID={id} games={data.games} competitionBaseUrl={baseUrl} extra={extraCards} />
  </>;
}
