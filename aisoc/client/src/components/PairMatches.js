import { faClock } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useState } from 'react';
import './PairMatches.scss';
import TextBox from './TextBox';


export default function PairMatches({ baseUrl, matches, MatchComponent }) {
  const [filter, setFilter] = useState('');

  return <div className="pair-matches">
    <TextBox
      type="text"
      placeholder="Filter by username"
      value={filter}
      setValue={setFilter}
    />

    <div className='pair-matches-entry pair-matches-header'>
      <span className="pair-matches-position">#</span>
      <span className="pair-matches-player-1">Player 1</span>
      <span className="pair-matches-player-2">Player 2</span>
      <span className="pair-matches-time">Completed <FontAwesomeIcon icon={faClock} size="sm" /></span>
      <span className="pair-matches-match-link">Match</span>
    </div>

    {matches.map((match, i) => <MatchComponent match={match} filter={filter} baseUrl={baseUrl} i={i} key={i} />)}
  </div>;
}
