import { faCheck, faClock, faMedal } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import classNames from 'classnames';
import TextBox from 'components/TextBox';
import { useAuth } from 'hooks/useAuth';
import { useState } from 'react';
import { Link } from 'react-router-dom';
import { formatTime } from 'utils/time';
import { roundScore } from '../utils';
import './Leaderboard.scss';


const PAGE_SIZE = 20;


function ClimateHackLeaderboardRow({ rank, score, user, time, baseUrl, highlightUser }) {
  return <div className='ch-leaderboard-entry'>
    <span className="ch-leaderboard-position">{rank}</span>
    <span className={`ch-leaderboard-username ${highlightUser ? 'ch-leaderboard-username-highlighted' : ''}`}><Link to={`${baseUrl}user/${user.name()}`}>{user.name()}</Link>&nbsp;{user.profile.extra.climatehack_finalist &&
    <span style={{ color:'#09f' }}><FontAwesomeIcon icon={faMedal} size="sm" /></span>} {user.profile.admin && <FontAwesomeIcon icon={faCheck} fixedWidth size="sm" />}</span>
    <span className="ch-leaderboard-university">{user.university().name}</span>
    <span className="ch-leaderboard-time">{formatTime(time)}</span>
    <span className="ch-leaderboard-score">{String(score ? roundScore(score / 10000000) : 0.0).padEnd(7, '0')}</span>
  </div>;
}


export default function Leaderboard({ baseUrl, leaderboard }) {
  const auth = useAuth();
  const [page, setPage] = useState(0);
  const [filter, setFilter] = useState('');
  const lowerFilter = filter.toLowerCase();

  leaderboard.forEach((e, i) => {
    e._rank = i + 1;
  });

  const filteredLeaderboard = leaderboard.filter(entry => (entry.user.name().toLowerCase().includes(lowerFilter) || entry.user.university().name.toLowerCase().includes(lowerFilter)));

  const pages = Math.ceil(filteredLeaderboard.length / PAGE_SIZE);

  return <div className="ch-leaderboard">
    <TextBox
      type="text"
      placeholder="Filter by username or university"
      value={filter}
      setValue={v => {
        setPage(0);
        setFilter(v);
      }}
    />

    <div className='ch-leaderboard-entry ch-leaderboard-header'>
      <span className="ch-leaderboard-position">#</span>
      <span className="ch-leaderboard-username">Username</span>
      <span className="ch-leaderboard-university">University</span>
      <span className="ch-leaderboard-time">Submitted <FontAwesomeIcon icon={faClock} size="sm" /></span>
      <span className="ch-leaderboard-score">Score</span>
    </div>

    {filteredLeaderboard.map((entry, i) => i >= page * PAGE_SIZE && i < (page + 1) * PAGE_SIZE && <ClimateHackLeaderboardRow
      key={i}
      rank={entry._rank}
      score={entry.score}
      user={entry.user}
      time={entry.uploaded_at}
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
