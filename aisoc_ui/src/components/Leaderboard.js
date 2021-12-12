import { useState } from 'react';
import { Link } from 'react-router-dom';
import './Leaderboard.scss';


export default function Leaderboard({ baseUrl }) {
  const leaderboard = [{ 'agent': 'a210675ab354f33d488f', 'score': 41, 'username': 'testaccount' }, { 'agent': 'c854ab2e9ce7f8c3dcdd', 'score': 39, 'username': 'louisdewardt' }];

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
      <span className="leaderboard-username">Username</span>
      <span className="leaderboard-score">Score</span>
    </div>

    {leaderboard.map((entry, i) => entry.username.includes(filter) && <div key={i} className='leaderboard-card'>
      <span className="leaderboard-position">{i + 1}</span>
      <span className="leaderboard-username"><Link to={`${baseUrl}user/${entry.username}`}>{entry.username}</Link></span>
      <span className="leaderboard-score">{entry.score}</span>
    </div>)}
  </div>;
}
