import { DoxaError } from 'api/common';
import GameCard from 'competitions/uttt/components/GameCard/GameCard';
import Button from 'components/Button';
import Card from 'components/Card';
import { useAuth } from 'hooks/useAuth';
import { useEffect, useState } from 'react';
import { Link, Redirect, useParams } from 'react-router-dom';
import UTTTAPI from '../../api';
<<<<<<< HEAD
=======
import CancelledCard from './CancelledCard';
>>>>>>> origin/master
import ErrorCard from './ErrorCard';
import './Match.scss';
import OngoingCard from './OngoingCard';
import TitleCard from './TitleCard';

async function loadMatchData(matchID, authToken) {
  const game = await UTTTAPI.getGame(matchID, null);
  const players = await UTTTAPI.getGamePlayers(matchID);
  const events = await UTTTAPI.getGameEvents(matchID, null, authToken);

  const games = [];

<<<<<<< HEAD
  let error, forfeit, scores;
=======
  let error, forfeit, scores, cancelled;
>>>>>>> origin/master

  for (let event of events) {
    if (event.type.startsWith('game_') && event.type != 'game_winners') {
      // NOTE: if we want to include the timestamp of the game in the UI,
      // we can copy game.timestamp into game.payload
      games.push(event.payload);
    } else if (event.type === '_ERROR') {
      error = event;
<<<<<<< HEAD
=======
    } else if (event.type === '_CANCELLED') {
      cancelled = event;
>>>>>>> origin/master
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

<<<<<<< HEAD
  return { games, queuedAt: game.queued_at, startedAt: game.started_at, completedAt: game.completed_at, error, forfeit, players, scores };
=======
  const cancelledAt = cancelled ? new Date(cancelled.timestamp) : null;

  return { games, queuedAt: game.queued_at, startedAt: game.started_at, completedAt: game.completed_at, cancelledAt, error, forfeit, players, scores };
>>>>>>> origin/master
}


export default function Match({ baseUrl }) {
  const { id } = useParams();
  const [data, setData] = useState(null);
  const auth = useAuth();

  const [error, setError] = useState(false);
  const [notFound, setNotFound] = useState(false);

  useEffect(async () => {
    try {
      setData(await loadMatchData(id, auth.token));
    } catch (e) {
      setError(true);
      if (e instanceof DoxaError) {
        if (e.status_code === 404) {
          setNotFound(true);
        }
      }
    }
  }, []);

  if (notFound) {
    return <Redirect to='/404' />;
  }

  if (error) {
    return <>
      <Card>
        <h1>Error</h1>
        <p>
          Sorry &mdash; an error occurred while loading this match.
        </p>
      </Card>
      <Link to={baseUrl}>
        <Button success>Return to the main competition page</Button>
      </Link>
    </>;
  }

  if (!data) {
    return <></>;
  }

  return <>
    <span></span><span></span><span></span><span></span> {/* a fun hack just to get a better outline colour below! */}
<<<<<<< HEAD
    <TitleCard players={data.players} scores={data.scores} completedAt={data.completedAt} queuedAt={data.queuedAt} startedAt={data.startedAt} baseUrl={baseUrl} />
    <h3 className="match-showing-n-games-label">Showing {data.games.length} game{data.games.length != 1 ? 's' : ''}</h3>
    <div className='match-games'>
      {data.games.map((game, i) => {
        return <GameCard key={i} matchID={id} gameID={i + 1} game={game} baseUrl={baseUrl} />;
      })}
      {!data.completedAt && <OngoingCard started={!!data.startedAt} />}
      {(data.forfeit || data.error) && <ErrorCard error={data.error} forfeit={data.forfeit} players={data.players} baseUrl={baseUrl} />}
=======
    <TitleCard players={data.players} scores={data.scores} completedAt={data.completedAt} queuedAt={data.queuedAt} startedAt={data.startedAt} cancelledAt={data.cancelledAt} baseUrl={baseUrl} />
    <h3 className="match-showing-n-games-label">Showing {data.games.length} game{data.games.length != 1 ? 's' : ''}</h3>
    <div className='match-games'>
      {data.games.map((game, i) => {
        return <GameCard key={i} matchID={id} gameID={i + 1} game={game} baseUrl={baseUrl} players={data.players} forfeit={data.forfeit} />;
      })}
      {!data.completedAt && <OngoingCard started={!!data.startedAt} />}
      {data.cancelledAt && <CancelledCard />}
      {(data.forfeit || data.error) && <ErrorCard error={data.error} forfeit={data.forfeit} players={data.players} baseUrl={baseUrl} canShowForfeitedError={data.games.length == 0} />}
>>>>>>> origin/master
    </div>
  </>;
}
