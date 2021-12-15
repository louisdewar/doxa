import Leaderboard from 'components/Leaderboard';
import { useEffect, useState } from 'react';
import UTTTAPI from '../api';


export default function LeaderboardTab({ baseUrl }) {
  const [leaderboard, setLeaderboard] = useState(null);

  useEffect(() => {
    UTTTAPI.getLeaderboardActive().then(leaderboardData => {
      setLeaderboard(leaderboardData);
    }).catch(err => {
      console.error(err);
    });
  }, []);

  return <div>
    <h2>Leaderboard</h2>
    {leaderboard && <Leaderboard baseUrl={baseUrl} leaderboard={leaderboard} />}
  </div>;
}
