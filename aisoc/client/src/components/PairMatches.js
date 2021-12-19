import { useState } from 'react';
import { Link } from 'react-router-dom';
import './Leaderboard.scss';
import TextBox from './TextBox';


export default function PairMatches({ baseUrl, matches }) {
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
      <span className="leaderboard-username">Users</span>
      <span className="leaderboard-match-link">Match</span>
    </div>

    {matches.map((entry, i) => (entry.player1.includes(filter) || entry.player2.includes(filter)) && <div key={i} className='leaderboard-entry'>
      <span className="leaderboard-position">{i + 1}</span>
      <span className="leaderboard-username">
        <Link to={`${baseUrl}user/${entry.player1}`}>{entry.player1}</Link> {entry.score1 && `(${entry.score1})`} vs <Link to={`${baseUrl}user/${entry.player2}`}>{entry.player2}</Link> {entry.score2 && `(${entry.score2})`}
      </span>
      <span className="leaderboard-match-link"><Link to={`${baseUrl}match/${entry.id}`}>View match</Link></span>
    </div>)}
  </div>;
}
