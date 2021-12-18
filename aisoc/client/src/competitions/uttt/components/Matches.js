import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import UTTTAPI from '../api';
import './Matches.scss';



export default function Matches({ username, competitionBaseUrl }) {
  const [filter, setFilter] = useState(null);
  const [matches, setMatches] = useState(null);

  useEffect(() => {
    UTTTAPI.getUserActiveGames(username)
      .then(async matches => {
        // const matchPlayers = await Promise.all(matches.map(match => api.getGamePlayers(match.id)));
        setMatches(matches);
      })
      .catch(err => {
        console.error(err);
      });
  }, [username]);

  return (
    <div className='matches maxwidth'>
      <h1>Matches</h1>
      <input type='text' placeholder='filter by username' onChange={e => setFilter(e.target.value)} />
      {/* {matches? matches.filter(players => filter == null || players[0].includes(filter) || players[1].includes(filter)).map(([player1, player2], i) => { */}
      {matches ? matches.map(match => {
        return <MatchCard key={match.id} matchID={match.id} filter={filter} mainPlayer={username} competitionBaseUrl={competitionBaseUrl} />;
      }) : 'Loading matches...'}

    </div>
  );
}

function MatchCard({ matchID, mainPlayer, filter, competitionBaseUrl }) {
  const [loaded, setLoaded] = useState(false);
  const [player, setPlayer] = useState(null);
  const [opponent, setOpponent] = useState(null);
  const [score, setScore] = useState(null);

  // Load player and opponent
  useEffect(() => {
    setLoaded(false);
    UTTTAPI.getGamePlayers(matchID).then(players => {
      setPlayer(players[0]);
      setOpponent(players[1]);

      let mainAgent;
      if (players[0].username === mainPlayer) {
        mainAgent = players[0].agent;
      } else if (players[1].username === mainPlayer) {
        mainAgent = players[1].agent;
      } else {
        throw new Error('Neither player 0 nor player 1 was the main player');
      }

      return UTTTAPI.getGameResult(matchID, mainAgent).then(result => {
        setScore(result);
        setLoaded(true);
      });
    })
      .catch(err => {
        console.error(err);
      });
  }, [matchID, mainPlayer]);


  if (!loaded) {
    return null;
  }

  if (!(!filter || player.username.includes(filter) || opponent.username.includes(filter))) {
    return null;
  }


  return (
    <Link to={`${competitionBaseUrl}match/${matchID}`}>
      <div className='match-card'>
        <div className='match-players'>
          <p className='username'>{player.username}</p>
          <p>VS</p>
          <p className='opponent'>{opponent.username}</p>
        </div>
        <p className='score'>{score} <span className='points'>points</span></p>
      </div>
    </Link>
  );
}
