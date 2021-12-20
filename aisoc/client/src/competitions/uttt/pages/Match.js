import Card from 'components/Card';
import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import { useAuth } from 'hooks/useAuth';
import UTTTAPI from '../api';
import Games from '../components/Games';
import './Match.scss';

import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faExclamationTriangle } from '@fortawesome/free-solid-svg-icons';
import PlayerLink from '../components/PlayerLink';

const PLAYER_CLASS = ['main', 'opposing'];

async function loadMatchData(matchID, authToken) {
  const winners = await UTTTAPI.getUTTTGameWinners(matchID);
  const scores = await UTTTAPI.getUTTTGameScores(matchID);
  const players = await UTTTAPI.getGamePlayers(matchID);

  const total = scores.a_wins + scores.b_wins + scores.draws;

  const forfeit = await UTTTAPI.getSingleGameEvent(matchID, '_FORFEIT', authToken);
  if (forfeit) {
    forfeit.payload.remaining = total - winners.length;
  }

  const calculatePercentage = number => 100 * number / total;
  scores.percentages = {
    a_wins: calculatePercentage(scores.a_wins),
    b_wins: calculatePercentage(scores.b_wins),
    draws: calculatePercentage(scores.draws)
  };

  return { winners, scores, players, forfeit };
}


function ForfeitCard({ stderr, players, forfeiter, remaining, baseUrl }) {
  const other = forfeiter === 0? 1: 0;

  let extraInfo;

  if (stderr) {
    extraInfo = (
      <>
        <p className="stderr-message">You have permission to view the <code>stderr</code> output of <PlayerLink username={players[forfeiter].username} baseUrl={baseUrl} playerClass={PLAYER_CLASS[forfeiter]} />&apos;s agent (max 50mb):</p>
        <pre className="stderr">{stderr}</pre>
      </>
    );
  }

  return (
    <div className="game-card forfeit">
      <div className="forfeit-icon"><FontAwesomeIcon icon={faExclamationTriangle} /></div>
      <p><PlayerLink username={players[forfeiter].username} baseUrl={baseUrl} playerClass={PLAYER_CLASS[forfeiter]} />&apos;s agent forfeit the match!</p>
      <p>
        This means that <PlayerLink username={players[other].username} baseUrl={baseUrl} playerClass={PLAYER_CLASS[other]} /> wins
        the remaining {remaining} {remaining > 1? 'games': 'game'} by default.
      </p>
      {extraInfo}
    </div>
  );
}


export default function Match({ baseUrl }) {
  const { id } = useParams();
  const [data, setData] = useState(null);
  const auth = useAuth();

  useEffect(async () => {
    setData(await loadMatchData(id, auth.token));
  }, []);

  if (!data) {
    return <></>;
  }

  let extraCards = null;

  if (data.forfeit) {
    const { agent, remaining, stderr } = data.forfeit.payload;
    extraCards = <ForfeitCard players={data.players} forfeiter={agent} remaining={remaining} stderr={stderr} />;
  }

  return <>
    <span></span><span></span><span></span><span></span> {/* a fun hack just to get a better outline colour below! */}

    <Card darker className="match-page-header">
      <h1><PlayerLink username={data.players[0].username} baseUrl={baseUrl} playerClass={'main'} /> vs <PlayerLink username={data.players[1].username} baseUrl={baseUrl} playerClass={'opposing'} />
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

    <Games matchID={id} winners={data.winners} competitionBaseUrl={baseUrl} extra={extraCards} />
  </>;
}
