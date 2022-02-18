import { faTimesCircle } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { DoxaError } from 'api/common';
import Card from 'components/Card';
import { useAuth } from 'hooks/useAuth';
import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import ClimateHackAPI from '../api';
import EvaluationLog from '../components/EvaluationLog';

export default function Game({ baseUrl }) {
  const auth = useAuth();
  const { id } = useParams();
  const [game, setGame] = useState(null);
  const [events, setEvents] = useState(null);
  const [errorMessage, setErrorMessage] = useState(null);

  useEffect(async () => {
    if (game !== null) {
      setGame(null);
    }

    if (events !== null) {
      setEvents(null);
    }

    try {
      setGame(await ClimateHackAPI.getGame(id));

      const eventData = await ClimateHackAPI.getGameEvents(id, undefined, auth.token);
      setEvents(eventData || []);
    } catch (e) {
      if (e instanceof DoxaError) {
        setErrorMessage(e.error_message);
      } else {
        setErrorMessage('Error');
      }
    }
  }, [id]);

  return <>
    <span></span><span></span><span></span><span></span> {/* a fun hack just to get a better outline colour below! */}
    <Card darker className='competitionHeader'>
      <h2>Submission #{id} {game && game.outdated && <FontAwesomeIcon icon={faTimesCircle} size='sm' fixedWidth />}</h2>
    </Card>

    {errorMessage && <Card>{errorMessage}</Card>}

    {events && <EvaluationLog game={game} events={events} baseUrl={baseUrl} />}
  </>;
}
