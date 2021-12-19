import LoadingPlaceholder from 'components/LoadingPlaceholder';
import human from 'human-time';
import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import UTTTAPI from '../api';

async function fetchMatchRow(setPlayers, setScores, matchID) {
  setPlayers(null);
  setScores(null);

  const players = await UTTTAPI.getGamePlayers(matchID);

  setPlayers(players.map(player => player.username));

  const score1 = await UTTTAPI.getGameResult(matchID, players[0].agent);
  const score2 = await UTTTAPI.getGameResult(matchID, players[1].agent);

  setScores([score1, score2]);
}

export default function MatchRow({ filter, match, baseUrl, i }) {
  const [players, setPlayers] = useState(null);
  const [scores, setScores] = useState(null);

  useEffect(async () => {
    await fetchMatchRow(setPlayers, setScores, match.id);
  }, [match.id]);

  let username;
  if (players) {
    const [player1, player2] = players;

    if (!player1.includes(filter) && !player2.includes(filter)) {
      return null;
    }

    if (scores) {
      const [score1, score2] = scores;

      username = (
        <>
          <Link to={`${baseUrl}user/${player1}`}>{player1}</Link> {' '}
          ({score1})
          vs {' '}
          <Link to={`${baseUrl}user/${player2}`}>{player2}</Link> {' '}
          ({score2})
        </>
      );
    } else {
      username = (
        <>
          <Link to={`${baseUrl}user/${player1}`}>{player1}</Link> {' '}
          (<LoadingPlaceholder height={13} width={20} />)
          vs {' '}
          <Link to={`${baseUrl}user/${player2}`}>{player2}</Link> {' '}
          (<LoadingPlaceholder height={13} width={20} />)
        </>
      );
    }
  } else {
    username = (
      <>
        <LoadingPlaceholder height={15} width={'100%'} />
      </>
    );
  }

  return (
    <div className='pair-matches-entry'>
      <span className="pair-matches-position">{i + 1}</span>
      <span className="pair-matches-username">{username}</span>
      <span className="pair-matches-time">{match.end_time ? human(new Date(match.end_time)) : <em>Ongoing</em>}</span>
      <span className="pair-matches-match-link"><Link to={`${baseUrl}match/${match.id}`}>View</Link></span>
    </div>
  );
}
