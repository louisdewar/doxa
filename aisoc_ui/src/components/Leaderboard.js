import { useState } from 'react';
import { Link } from 'react-router-dom';
import './Leaderboard.scss';
import TextBox from './TextBox';


export default function Leaderboard({ baseUrl, leaderboard }) {
  const [filter, setFilter] = useState('');

  return <div className="leaderboard">
    <TextBox
      type="text"
      placeholder="Filter by username"
      value={filter}
      setValue={setFilter}
    />

    <div className='leaderboard-entry leaderboard-header'>
      <span className="leaderboard-position">#</span>
      <span className="leaderboard-username">User</span>
      <span className="leaderboard-score">Score</span>
    </div>

    {leaderboard.map((entry, i) => entry.username.includes(filter) && <div key={i} className='leaderboard-entry'>
      <span className="leaderboard-position">{i + 1}</span>
      <span className="leaderboard-username"><Link to={`${baseUrl}user/${entry.username}`}>{entry.username}</Link></span>
      <span className="leaderboard-score">{entry.score}</span>
    </div>)}
  </div>;
}
