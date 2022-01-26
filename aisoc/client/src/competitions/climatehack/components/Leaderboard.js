import TextBox from 'components/TextBox';
import { useState } from 'react';
import './Leaderboard.scss';

function roundScore(score) {
  return Math.round((score + Number.EPSILON) * 100000) / 100000;
}

function ClimateHackLeaderboardRow({  rank, score, user }) {
  return <div className='ch-leaderboard-entry'>
    <span className="ch-leaderboard-position">{rank}</span>
    <span className="ch-leaderboard-username">{user.name()}</span>
    <span className="ch-leaderboard-university">{user.university().name}</span>
    <span className="ch-leaderboard-score">{score ? roundScore(score / 10000000) : 0}</span>
  </div>;
}


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

    {leaderboard.map((entry, i) => (entry.user.name().includes(filter) || entry.user().university().name.includes(filter)) && <ClimateHackLeaderboardRow rank={i+1} score={entry.score} user={entry.user} />)}
  </div>;
}
