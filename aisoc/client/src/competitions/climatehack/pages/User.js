import Card from 'components/Card';
import { useAuth } from 'hooks/useAuth';
import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import ClimateHackAPI from '../api';
import EvaluationLog from '../components/EvaluationLog';
import { roundScore } from '../utils';

export default function User() {
  const auth = useAuth();
  const { user } = useParams();
  const [score, setScore] = useState(null);
  const [events, setEvents] = useState(null);

  useEffect(async () => {
    if (score !== null) {
      setScore(null);
    }

    const data = await ClimateHackAPI.getUserScore(user, 'dataset_dapper');
    setScore(data.score || 0);

    if (events !== null) {
      setEvents(null);
    }

    const eventData = await ClimateHackAPI.getActiveGameEvents(user, auth.token);
    setEvents(eventData || []);
    console.log(events);
  }, [user]);

  return <>
    <span></span><span></span><span></span><span></span> {/* a fun hack just to get a better outline colour below! */}
    <Card darker className='competitionHeader'>
      <h1>{user}</h1>
      <h2>
        {roundScore(score / 10000000)}
      </h2>
    </Card>

    {events && <EvaluationLog events={events} />}
  </>;
}
