import { useState } from 'react';
import { Link } from 'react-router-dom';
import './Leaderboard.scss';
import TextBox from './TextBox';


export default function PairMatches({ baseUrl, matches }) {
  // const matches = [
  //   { player1: 'louisdewardt', player2: 'testaccount', score: 21 },
  //   { player1: 'testaccount', player2: 'louisdewardt', score: 20 }
  // ];

  const [filter, setFilter] = useState('');

  return <div className="leaderboard">
    <TextBox
      type="text"
      placeholder="Filter by username"
      value={filter}
      setValue={setFilter}
    />

    <div className='leaderboard-entry leaderboard-entry-header'>
      <span className="leaderboard-position">#</span>
      <span className="leaderboard-username">Users</span>
      <span className="leaderboard-score">Score</span>
    </div>

    {matches.map((entry, i) => (entry.player1.includes(filter) || entry.player2.includes(filter)) && <div key={i} className='leaderboard-entry'>
      <span className="leaderboard-position">{i + 1}</span>
      <span className="leaderboard-username"><Link to={`${baseUrl}user/${entry.player1}`}>{entry.player1}</Link> vs <Link to={`${baseUrl}user/${entry.player2}`}>{entry.player2}</Link></span>
      <span className="leaderboard-score">{entry.score}</span>
    </div>)}
  </div>;
}
