import api from 'common/api';
import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import './Leaderboard.scss';



export default function Leaderboard() {
  const [filter, setFilter] = useState(null);
  const [leaderboard, setLeaderboard] = useState(null);

  useEffect(() => {
    api.leaderboard.getActive().then(leaderboardData => {
      setLeaderboard(leaderboardData);
    }).catch(err => {
      console.error(err);
    });
  }, []);

  return (
    <div className="leaderboard maxwidth">
      <h1>Leaderboard</h1>

      <input type='text' placeholder='filter by username' onChange={e => setFilter(e.target.value)} />
      {leaderboard ? leaderboard.map((player, i) => {
        return <LeaderboardCard key={i} rank={i + 1} username={player.username} score={player.score} filter={filter} />;
      }) : 'Loading leaderboard...'}

    </div>
  );
}

function LeaderboardCard({ rank, username, score, filter }) {
  if (filter !== null && !username.includes(filter)) {
    return null;
  }

  return (
    <Link to={'/c/uttt/user/' + username}>
      <div className='leaderboard-card'>
        <p className='rank-username'>#{rank} {username}</p>
        <p className='score'>{score} <span className='points'>points</span></p>
      </div>
    </Link>
  );
}
