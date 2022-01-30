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
  const { game } = useParams();
  const [gameData, setGameData] = useState(null);
  const [events, setEvents] = useState(null);
  const [errorMessage, setErrorMessage] = useState(null);

  useEffect(async () => {
    if (gameData !== null) {
      setGameData(null);
    }

    if (events !== null) {
      setEvents(null);
    }

    try {
      setGameData(await ClimateHackAPI.getGame(game));

      const eventData = await ClimateHackAPI.getGameEvents(game, undefined, auth.token);
      setEvents(eventData || []);
    } catch (e) {
      if (e instanceof DoxaError) {
        setErrorMessage(e.error_message);
      } else {
        setErrorMessage('Error');
      }
    }
  }, [game]);

  return <>
    <span></span><span></span><span></span><span></span> {/* a fun hack just to get a better outline colour below! */}
    <Card darker className='competitionHeader'>
      <h2>Game #{game} {gameData && gameData.outdated && <FontAwesomeIcon icon={faTimesCircle} size='sm' fixedWidth />}</h2>
      <div style={{ fontSize: '0.7rem' }}>
        <p>
          {gameData && <>
            {gameData.queued_at && <h2>Queued at {gameData.queued_at.toString()}</h2>}
          </>}
        </p>
        <p>
          {gameData && <>
            {gameData.started_at && <h2>Started at {gameData.started_at.toString()}</h2>}
          </>}
        </p>
        <p>
          {gameData && <>
            {gameData.completed_at && <h2>Completed at {gameData.completed_at.toString()}</h2>}
          </>}
        </p>
      </div>
    </Card>

    {errorMessage && <Card>{errorMessage}</Card>}

    {events && <EvaluationLog events={events} baseUrl={baseUrl} />}
  </>;
}
