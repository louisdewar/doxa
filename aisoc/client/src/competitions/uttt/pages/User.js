import Card from 'components/Card';
import LoadingPlaceholder from 'components/LoadingPlaceholder';
import PairMatches from 'components/PairMatches';
import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import UTTTAPI from '../api';
import { Link } from 'react-router-dom';

async function fetchMatchRow(setPlayers, setScores, matchID) {
  setPlayers(null);
  setScores(null);

  const players = await UTTTAPI.getGamePlayers(matchID);

  setPlayers(players.map(player => player.username));

  const score1 = await UTTTAPI.getGameResult(matchID, players[0].agent);
  const score2 = await UTTTAPI.getGameResult(matchID, players[1].agent);

  setScores([score1, score2]);
}

function MatchRow({ filter, matchID, baseUrl, i }) {
  const [players, setPlayers] = useState(null);
  const [scores, setScores] = useState(null);

  useEffect(async () => {
    await fetchMatchRow(setPlayers, setScores, matchID);
  }, [matchID]);

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
    <div className='leaderboard-entry'>
      <span className="leaderboard-position">{i + 1}</span>
      <span className="leaderboard-username">{username}</span>
      <span className="leaderboard-match-link"><Link to={`${baseUrl}match/${matchID}`}>View match</Link></span>
    </div>
  );
}

export default function User({ baseUrl }) {
  const { user } = useParams();
  const [score, setScore] = useState(null);
  const [matchIDs, setMatchIDs] = useState([]);

  useEffect(async () => {
    if (score !== null) {
      setScore(null);
    }

    const data = await UTTTAPI.getUserScore(user);
    setScore(data.score || 0);
  }, [user]);

  useEffect(async () => {
    if (matchIDs !== []) {
      setMatchIDs([]);
    }

    const activeGames = await UTTTAPI.getUserActiveGames(user);
    setMatchIDs(activeGames.map(game => game.id));
  }, [user]);

  return <>
    <Card darker className='competitionHeader'>
      <h1>{user}</h1>
      <h2>
        {score === null? <LoadingPlaceholder height={25} width={35} />: score} points
      </h2>
    </Card>
    <Card>
      <h2>Matches</h2>
      <PairMatches baseUrl={baseUrl} matchIDs={matchIDs} MatchComponent={MatchRow} />
    </Card>
  </>;
}
