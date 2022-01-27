import TextBox from 'components/TextBox';
import { useState } from 'react';
import { roundScore } from '../utils';
import './Leaderboard.scss';

function ClimateHackLeaderboardRow({ rank, score, user }) {
  return <div className='ch-leaderboard-entry'>
    <span className="ch-leaderboard-position">{rank}</span>
    <span className="ch-leaderboard-username">{user.name()}</span>
    <span className="ch-leaderboard-university">{user.university().name}</span>
    <span className="ch-leaderboard-score">{score ? roundScore(score / 10000000) : 0}</span>
  </div>;
}


export default function Leaderboard({ leaderboard }) {
  const [filter, setFilter] = useState('');

  const lowerFilter = filter.toLowerCase();

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

    {leaderboard.map((entry, i) => (entry.user.name().toLowerCase().includes(lowerFilter) || entry.user.university().name.toLowerCase().includes(lowerFilter)) && <ClimateHackLeaderboardRow key={i} rank={i + 1} score={entry.score} user={entry.user} />)}
  </div>;
}
