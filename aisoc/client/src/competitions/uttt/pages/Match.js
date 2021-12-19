import Card from 'components/Card';
import { useEffect, useState } from 'react';
import { Link, useParams } from 'react-router-dom';
import UTTTAPI from '../api';
import Games from '../components/Games';
import './Match.scss';

async function loadMatchData(matchID) {
  const winners = await UTTTAPI.getUTTTGameWinners(matchID);
  const scores = await UTTTAPI.getUTTTGameScores(matchID);
  const players = await UTTTAPI.getGamePlayers(matchID);

  const total = scores.a_wins + scores.b_wins + scores.draws;
  const calculatePercentage = number => 100 * number / total;
  scores.percentages = {
    a_wins: calculatePercentage(scores.a_wins),
    b_wins: calculatePercentage(scores.b_wins),
    draws: calculatePercentage(scores.draws)
  };

  return { winners, scores, players };
}

export default function Match({ baseUrl }) {
  const { id } = useParams();
  const [data, setData] = useState(null);

  useEffect(async () => {
    setData(await loadMatchData(id));
  }, []);

  if (!data) {
    return <></>;
  }

  return <>
    <span></span><span></span><span></span><span></span> {/* a fun hack just to get a better outline colour below! */}

    <Card darker className="match-page-header">
      <h1>
        <Link to={`${baseUrl}user/${data.players[0].username}`} className="match-page-main-player-link">{data.players[0].username}</Link> vs <Link to={`${baseUrl}user/${data.players[1].username}`} className="match-page-opposing-player-link">{data.players[1].username}</Link>
      </h1>
      <h2>
        {data.scores.a_wins} wins | {data.scores.draws} draws | {data.scores.b_wins} losses
      </h2>
      <div className='match-score-bar'>
        {data.scores.percentages.a_wins > 0 && <div className='match-score-bar-wins' style={{ width: data.scores.percentages.a_wins + '%' }}></div>}
        {data.scores.percentages.draws > 0 && <div className='match-score-bar-draws' style={{ width: data.scores.percentages.draws + '%' }}></div>}
        {data.scores.percentages.b_wins > 0 && <div className='match-score-bar-losses' style={{ width: data.scores.percentages.b_wins + '%' }}></div>}
      </div>
    </Card>

    <Games matchID={id} winners={data.winners} competitionBaseUrl={baseUrl} />
  </>;
}
