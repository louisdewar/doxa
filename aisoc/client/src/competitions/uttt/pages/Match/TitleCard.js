import { faClock, faHourglassEnd } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import Card from 'components/Card';
import { formatDuration, formatTime } from 'utils/time';
import PlayerLink from '../../components/PlayerLink/PlayerLink';
import './Match.scss';

export default function TitleCard({ players, scores, baseUrl, completedAt, cancelledAt, queuedAt, startedAt }) {
  const duration = completedAt ? formatDuration((completedAt.getTime() - startedAt.getTime()) / 1000) : null;
  const end = cancelledAt ? `was cancelled ${formatTime(cancelledAt)}`: (completedAt ? `finished ${formatTime(completedAt)}` :
    (startedAt ? `started ${formatTime(startedAt)}` : `was queued ${formatTime(queuedAt)}`));

  return <Card darker className='match-page-header'>
    <h1>
      <PlayerLink username={players[0].username} baseUrl={baseUrl} playerClass='main' /> vs <PlayerLink username={players[1].username} baseUrl={baseUrl} playerClass='opposing' />
    </h1>

    {scores ? <>
      <h2>
        {scores.a_wins} wins | {scores.draws} draws | {scores.b_wins} losses
      </h2>
      <div className='match-score-bar'>
        {scores.percentages.a_wins > 0 && <div className='match-score-bar-wins' style={{ width: scores.percentages.a_wins + '%' }}></div>}
        {scores.percentages.draws > 0 && <div className='match-score-bar-draws' style={{ width: scores.percentages.draws + '%' }}></div>}
        {scores.percentages.b_wins > 0 && <div className='match-score-bar-losses' style={{ width: scores.percentages.b_wins + '%' }}></div>}
      </div>
    </> : <h2>No scores</h2>}

    <p className='completed'>
      <FontAwesomeIcon icon={faClock} size='sm' fixedWidth /> This match {end}.<br />
      {duration && <><FontAwesomeIcon icon={faHourglassEnd} size='sm' fixedWidth /> This match took {duration} to complete.</>}
    </p>
  </Card>;
}
