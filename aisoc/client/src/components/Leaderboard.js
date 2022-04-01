import { useState } from 'react';
import { Link } from 'react-router-dom';
import './Leaderboard.scss';
import TextBox from './TextBox';

function DefaultLeaderboardRow({ baseUrl, rank, score, user }) {
  return <div className='leaderboard-entry'>
    <span className="leaderboard-position">{rank}</span>
    <span className="leaderboard-username"><Link to={`${baseUrl}user/${user.name()}`}>{user.name()}</Link></span>
    <span className="leaderboard-university">{user.university().name}</span>
    <span className="leaderboard-score">{score}</span>
  </div>;
}

export default function Leaderboard({ baseUrl, leaderboard, LeaderboardRow = DefaultLeaderboardRow }) {
  const [filter, setFilter] = useState('');
  const lowerFilter = filter.toLowerCase();

  return <div className="leaderboard">
    <TextBox
      type="text"
      placeholder="Filter by username and university"
      value={filter}
      setValue={setFilter}
    />

    <div className='leaderboard-entry leaderboard-header'>
      <span className="leaderboard-position">#</span>
      <span className="leaderboard-username">User</span>
      <span className="leaderboard-university">University</span>
      <span className="leaderboard-score">Score</span>
    </div>

    {leaderboard.map((entry, i) => (entry.user.name().toLowerCase().includes(lowerFilter) || entry.user.university().name.toLowerCase().includes(lowerFilter)) && <LeaderboardRow key={i} baseUrl={baseUrl} rank={i + 1} score={entry.score} user={entry.user} agent={entry.agent} />)}
  </div>;
}
