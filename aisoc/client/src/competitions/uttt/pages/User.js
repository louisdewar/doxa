import Card from 'components/Card';
import PairMatches from 'components/PairMatches';
import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import UTTTAPI from '../api';

export default function User({ baseUrl }) {
  const { user } = useParams();
  const [score, setScore] = useState(0);
  const [matches, setMatches] = useState(null);

  useEffect(async () => {
    const data = await UTTTAPI.getUserScore(user);
    setScore(data.score || 0);
  }, [user]);

  useEffect(async () => {
    const activeGames = await UTTTAPI.getUserActiveGames(user);
    const matches = [];

    for (const activeGame of activeGames) {
      const players = await UTTTAPI.getGamePlayers(activeGame.id);
      const mainAgent = (players[0].username == user ? players[0] : players[1]).agent;
      const otherAgent = (players[0].username != user ? players[0] : players[1]).agent;
      const score1 = await UTTTAPI.getGameResult(activeGame.id, mainAgent);
      const score2 = await UTTTAPI.getGameResult(activeGame.id, otherAgent);

      matches.push({
        id: activeGame.id,
        player1: players[0].username,
        player2: players[1].username,
        score1,
        score2
      });
    }

    setMatches(matches);
  }, [user]);

  return <>
    <Card darker className='competitionHeader'>
      <h1>{user}</h1>
      <h2>
        {score} points
      </h2>
    </Card>
    {matches && <Card>
      <h2>Matches</h2>
      <PairMatches baseUrl={baseUrl} matches={matches} />
    </Card>}
  </>;
}
