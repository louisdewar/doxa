import { useState } from 'react';
import { Link } from 'react-router-dom';
import './Leaderboard.scss';


export default function Matches({ baseUrl }) {
  const matches = [
    { username: 'louisdewardt', score: 21 },
    { username: 'testaccount', score: 20 }
  ];

  const [filter, setFilter] = useState('');
  const handleFilterChange = e => {
    setFilter(e.target.value);
  };


  return <div className="leaderboard">
    <input
      type="text"
      className='leaderboard-filter'
      placeholder='Filter by username'
      value={filter}
      onChange={handleFilterChange}
    />

    <div className='leaderboard-card leaderboard-card-header'>
      <span className="leaderboard-position">#</span>
      <span className="leaderboard-username">User</span>
      <span className="leaderboard-score">Score</span>
    </div>

    {matches.map((entry, i) => entry.username.includes(filter) && <div key={i} className='leaderboard-card'>
      <span className="leaderboard-position">{i + 1}</span>
      <span className="leaderboard-username"><Link to={`${baseUrl}user/${entry.username}`}>{entry.username}</Link></span>
      <span className="leaderboard-score">{entry.score}</span>
    </div>)}
  </div>;
}
