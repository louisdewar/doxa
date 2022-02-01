import { DoxaError } from 'api/common';
import Card from 'components/Card';
import { useAuth } from 'hooks/useAuth';
import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import ClimateHackAPI from '../api';
import EvaluationLog from '../components/EvaluationLog';
import { roundScore } from '../utils';

export default function User({ baseUrl }) {
  const auth = useAuth();
  const { user } = useParams();
  const [score, setScore] = useState(null);
  const [game, setGame] = useState(null);
  const [events, setEvents] = useState(null);
  const [error, setError] = useState(false);
  const [errorMessage, setErrorMessage] = useState(null);

  useEffect(async () => {
    if (score !== null) {
      setScore(null);
    }

    try {
      const data = await ClimateHackAPI.getUserScore(user, 'dataset_dapper');
      setScore(data.score || 0);
    } catch (e) {
      setError(true);
      if (e instanceof DoxaError) {
        if (e.error_code == 'NO_ACTIVE_AGENT') {
          setErrorMessage('There is no active submission for this user.');
        } else if (e.error_code == 'USER_NOT_FOUND') {
          setErrorMessage('This user does not exist :-(');
        } else {
          setErrorMessage(e.error_message);
        }
      }
      return;
    }

    if (game !== null) {
      setGame(null);
    }

    if (events !== null) {
      setEvents(null);
    }

    try {
      const id = await ClimateHackAPI.getActiveGameId(user);

      if (id !== null && id !== undefined) {
        setGame(await ClimateHackAPI.getGame(id));

        const gameEvents = await ClimateHackAPI.getGameEvents(id, undefined, auth.token);
        if (gameEvents) {
          setEvents(gameEvents || []);
        }
      }

    } catch (e) {
      setErrorMessage(e.error_message);
    }
  }, [user]);

  if (error) {
    return <Card>{errorMessage}</Card>;
  }

  return <>
    <span></span><span></span><span></span><span></span> {/* a fun hack just to get a better outline colour below! */}
    <Card darker className='competitionHeader'>
      <h1>{user}</h1>
      <h2>
        {score && roundScore(score / 10000000)}
      </h2>
    </Card>

    {events && <EvaluationLog game={game} events={events} baseUrl={baseUrl} />}
  </>;
}
