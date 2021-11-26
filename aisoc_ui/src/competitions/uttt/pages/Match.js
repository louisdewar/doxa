import Games from 'competitions/uttt/components/Games.js';
import Navbar from 'components/NavBar.js';
import { useEffect, useState } from 'react';
import { useParams } from 'react-router';
import { Link } from 'react-router-dom';
import UTTTAPI from '../api';
import './Match.scss';



async function loadMatchData(matchID) {
  const winners = await UTTTAPI.getUTTTGameWinners(matchID);
  const scores = await UTTTAPI.getUTTTGameScores(matchID);
  const players = await UTTTAPI.getGamePlayers(matchID);

  const total = scores.a_wins + scores.b_wins + scores.draws;
  const calcPercentage = number => 100 * number / total;
  scores.percentages = {
    a_wins: calcPercentage(scores.a_wins),
    b_wins: calcPercentage(scores.b_wins),
    draws: calcPercentage(scores.draws)
  };

  return { winners, scores, players };
}

export default function Match({ competitionBaseUrl }) {
  const { matchID } = useParams();

  const [data, setData] = useState(null);

  useEffect(() => {
    loadMatchData(matchID).then(data => {
      setData(data);
    }).catch(err => {
      console.error(err);
    });
  }, [matchID]);

  if (!data) {
    return null;
  }

  const { winners, scores, players } = data;

  return (
    <div>
      <Navbar competitionName='Ultimate Tic-Tac-Toe' homepageUrl={competitionBaseUrl} />
      <div className="match-data">
        <div className="maxwidth">
          <div className="header-wrapper">
            <div className="player-versus">
              <Link to={`${competitionBaseUrl}user/${players[0].username}`}><span className="player player-1">{players[0].username}</span></Link>
              <span className="separator">VS</span>
              <Link to={`${competitionBaseUrl}user/${players[1].username}`}><span className="player player-2">{players[1].username}</span></Link>
            </div>
            <div className="scores">
              <span>wins</span>
              <span></span>
              <span>draws</span>
              <span></span>
              <span>losses</span>
              <span className="wins">{data.scores.a_wins}</span>
              <span className="score-separator">|</span>
              <span className="draws">{data.scores.draws}</span>
              <span className="score-separator">|</span>
              <span className="losses">{data.scores.b_wins}</span>
            </div>
          </div>
          <div className="score-bar">
            <div className="wins" style={{ width: scores.percentages.a_wins + '%' }}></div>
            <div className="draws" style={{ width: scores.percentages.draws + '%' }}></div>
            <div className="losses" style={{ width: scores.percentages.b_wins + '%' }}></div>
          </div>
        </div>
      </div>
      <Games matchID={matchID} winners={winners} competitionBaseUrl={competitionBaseUrl} />
    </div>
  );
}
