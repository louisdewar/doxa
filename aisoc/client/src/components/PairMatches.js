import { useState, Fragment } from 'react';
import './Leaderboard.scss';
import TextBox from './TextBox';

export default function PairMatches({ baseUrl, matchIDs, MatchComponent }) {
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

    {matchIDs.map((matchID, i) => <Fragment key={i}><MatchComponent matchID={matchID} filter={filter} baseUrl={baseUrl} i={i} /></Fragment>)}
  </div>;
}
