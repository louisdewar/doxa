import LoadingPlaceholder from 'components/LoadingPlaceholder';
import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { formatTime } from 'utils/time';
import UTTTAPI from '../../api';

async function fetchMatchRow(setPlayers, setScores, matchID) {
  setPlayers([undefined, undefined]);
  setScores([undefined, undefined]);

  const players = await UTTTAPI.getGamePlayers(matchID);

  setPlayers(players.map(player => player.username));

  const score1 = await UTTTAPI.getGameResult(matchID, players[0].agent);
  const score2 = await UTTTAPI.getGameResult(matchID, players[1].agent);

  setScores([score1, score2]);
}

function PlayerName({ baseUrl, player, score }) {
  if (!player) {
    return <LoadingPlaceholder height={15} width={'80%'} />;
  }

  return <>
    <Link to={`${baseUrl}user/${player}`}>{player}</Link> {score !== undefined
      ? (score !== null && `(${score})`)
      : <>(<LoadingPlaceholder height={13} width={20} />)</>
    }
  </>;
}

export default function MatchRow({ filter, match, baseUrl, i }) {
  const [players, setPlayers] = useState([undefined, undefined]);
  const [scores, setScores] = useState([undefined, undefined]);

  useEffect(async () => {
    await fetchMatchRow(setPlayers, setScores, match.id);
  }, [match.id]);

  if (players && players.length == 2 && players[0] && players[1] && !players[0].includes(filter) && !players[1].includes(filter)) {
    return null;
  }

  // let username;
  // if (players) {
  //   const [player1, player2] = players;

  //   if (!player1.includes(filter) && !player2.includes(filter)) {
  //     return null;
  //   }

  //   if (scores) {
  //     const [score1, score2] = scores;

  //     username = (
  //       <>
  //         <Link to={`${baseUrl}user/${player1}`}>{player1}</Link> {' '}
  //         <Score val={score1} />
  //         vs {' '}
  //         <Link to={`${baseUrl}user/${player2}`}>{player2}</Link> {' '}
  //         <Score val={score2} />
  //       </>
  //     );
  //   } else {
  //     username = (
  //       <>
  //         <Link to={`${baseUrl}user/${player1}`}>{player1}</Link> {' '}
  //         (<LoadingPlaceholder height={13} width={20} />)
  //         vs {' '}
  //         <Link to={`${baseUrl}user/${player2}`}>{player2}</Link> {' '}
  //         (<LoadingPlaceholder height={13} width={20} />)
  //       </>
  //     );
  //   }
  // } else {
  //   username = (
  //     <>
  //       <LoadingPlaceholder height={15} width={'80%'} />
  //     </>
  //   );
  // }

  const completed = match.completed_at ? formatTime(new Date(match.completed_at)) : (match.started_at ? <em>Ongoing</em> : <em>Queued</em>);

  return (
    <div className='pair-matches-entry'>
      <span className="pair-matches-position">{i + 1}</span>
      <span className="pair-matches-player-1"><PlayerName baseUrl={baseUrl} player={players[0]} score={scores[0]} /></span>
      <span className="pair-matches-player-2"><PlayerName baseUrl={baseUrl} player={players[1]} score={scores[1]} /></span>
      <span className="pair-matches-time">{completed}</span>
      <span className="pair-matches-match-link"><Link to={`${baseUrl}match/${match.id}`}>View</Link></span>
    </div>
  );
}
