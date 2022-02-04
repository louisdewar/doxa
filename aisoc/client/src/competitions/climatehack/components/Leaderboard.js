import classNames from 'classnames';
import TextBox from 'components/TextBox';
import { useAuth } from 'hooks/useAuth';
import { useState } from 'react';
import { Link } from 'react-router-dom';
import { roundScore } from '../utils';
import './Leaderboard.scss';

const PAGE_SIZE = 20;


function ClimateHackLeaderboardRow({ rank, score, user, baseUrl, highlightUser }) {
  return <div className='ch-leaderboard-entry'>
    <span className="ch-leaderboard-position">{rank}</span>
    <span className={`ch-leaderboard-username ${highlightUser ? 'ch-leaderboard-username-highlighted' : ''}`}><Link to={`${baseUrl}user/${user.name()}`}>{user.name()}</Link></span>
    <span className="ch-leaderboard-university">{user.university().name}</span>
    <span className="ch-leaderboard-score">{String(score ? roundScore(score / 10000000) : 0.0).padEnd(7, '0')}</span>
  </div>;
}


export default function Leaderboard({ baseUrl, leaderboard }) {
  const auth = useAuth();
  const [page, setPage] = useState(0);
  const [filter, setFilter] = useState('');
  const lowerFilter = filter.toLowerCase();

  const pages = Math.ceil(leaderboard.length / PAGE_SIZE);

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

    {leaderboard.map((entry, i) => i >= page * PAGE_SIZE && i < (page + 1) * PAGE_SIZE && (entry.user.name().toLowerCase().includes(lowerFilter) || entry.user.university().name.toLowerCase().includes(lowerFilter)) && <ClimateHackLeaderboardRow
      key={i}
      rank={i + 1}
      score={entry.score}
      user={entry.user}
      baseUrl={baseUrl}
      highlightUser={auth.user && auth.user.username && auth.user.username == entry.user}
    />)}

    {pages > 1 && <div className='ch-leaderboard-pagination'>
      Pages: {[...Array(pages).keys()]
        .map(n => <a key={n} onClick={() => setPage(n)} className={classNames({ 'ch-leaderboard-pagination-active': page == n })}>{n + 1}</a>)
        .reduce((prev, curr) => [prev, ', ', curr])}
    </div>}
  </div>;
}
