import TextBox from 'components/TextBox';
import { useState } from 'react';
import './Leaderboard.scss';


export default function Leaderboard({ leaderboard }) {
  const [filter, setFilter] = useState('');

  return <div className="ch-leaderboard">
    <TextBox
      type="text"
      placeholder="Filter by username or university"
      value={filter}
      setValue={setFilter}
    />

    <div className='ch-leaderboard-entry ch-leaderboard-header'>
      <span className="ch-leaderboard-position">#</span>
      <span className="ch-leaderboard-username">Username</span>
      <span className="ch-leaderboard-university">University</span>
      <span className="ch-leaderboard-score">Score</span>
    </div>

    {leaderboard.map((entry, i) => (entry.username.includes(filter) || entry.university.includes(filter)) && <div key={i} className='ch-leaderboard-entry'>
      <span className="ch-leaderboard-position">{i + 1}</span>
      {/* <Link to={`${baseUrl}user/${entry.username}`}>{entry.username}</Link> */}
      <span className="ch-leaderboard-username">{entry.username}</span>
      <span className="ch-leaderboard-university">{entry.university}</span>
      <span className="ch-leaderboard-score">{entry.score}</span>
    </div>)}
  </div>;
}
