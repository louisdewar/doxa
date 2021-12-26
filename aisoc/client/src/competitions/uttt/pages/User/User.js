import Card from 'components/Card';
import LoadingPlaceholder from 'components/LoadingPlaceholder';
import PairMatches from 'components/PairMatches';
import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import UTTTAPI from '../../api';
import MatchRow from '../../components/MatchRow';

export default function User({ baseUrl }) {
  const { user } = useParams();
  const [score, setScore] = useState(null);
  const [matches, setMatches] = useState([]);

  useEffect(async () => {
    if (score !== null) {
      setScore(null);
    }

    const data = await UTTTAPI.getUserScore(user);
    setScore(data.score || 0);
  }, [user]);

  useEffect(async () => {
    if (matches !== []) {
      setMatches([]);
    }

    setMatches(await UTTTAPI.getUserActiveGames(user));
  }, [user]);

  return <>
    <Card darker className='competitionHeader'>
      <h1>{user}</h1>
      <h2>
        {score === null ? <LoadingPlaceholder height={25} width={35} /> : score} points
      </h2>
    </Card>
    <Card>
      <h2>Matches</h2>
      <PairMatches baseUrl={baseUrl} matches={matches} MatchComponent={MatchRow} />
    </Card>
  </>;
}
