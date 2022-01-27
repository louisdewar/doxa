import { faBan, faClock, faExclamationTriangle, faFlagCheckered, faHourglassEnd, faHourglassStart } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { formatTime } from 'utils/time';
import { roundScore } from '../utils';
import './EvaluationLog.scss';



function EvaluationLogCard({ event, hasForfeited }) {
  if (event.type == '_START' || event.type == '_END') {
    return <div className='ch-evaluation-log-card ch-evaluation-log-endpoint'>
      <div className='ch-evaluation-log-card-body'>
        <FontAwesomeIcon icon={event.type == '_START' ? faHourglassStart : faHourglassEnd} size='sm' fixedWidth /> {event.type == '_START' ? 'This submission was received' : 'Evaluation terminated'} {formatTime(new Date(event.timestamp))}.
      </div>
    </div>;
  } else if (event.type == 'final') {
    return <div className='ch-evaluation-log-card'>
      <div className='ch-evaluation-log-card-body'>
        <FontAwesomeIcon icon={faClock} size='sm' fixedWidth /> The final score for this submission is {roundScore(event.payload.score)}.
      </div>
    </div>;
  } else if (event.type == '_FORFEIT') {
    return <div className='ch-evaluation-log-card ch-evaluation-log-forfeit'>
      <div className='ch-evaluation-log-card-body'>
        <FontAwesomeIcon icon={faExclamationTriangle} size='sm' fixedWidth /> An error occurred while running this submission.
      </div>
    </div>;
  } else if (event.type == '_ERROR' && !hasForfeited) {
    return <div className='ch-evaluation-log-card ch-evaluation-log-forfeit'>
      <div className='ch-evaluation-log-card-body'>
        <FontAwesomeIcon icon={faExclamationTriangle} size='sm' fixedWidth /> An internal server error occurred during the evaluation of this submission.
      </div>
    </div>;
  } else if (event.type == '_CANCELLED') {
    return <div className='ch-evaluation-log-card ch-evaluation-log-cancelled'>
      <div className='ch-evaluation-log-card-body'>
        <FontAwesomeIcon icon={faBan} size='sm' fixedWidth /> The evaluation of this submission was cancelled.
      </div>
    </div>;
  } else if (event.type.startsWith('checkpoint')) {
    return <div className='ch-evaluation-log-card'>
      <div className='ch-evaluation-log-card-body'>
        <FontAwesomeIcon icon={faFlagCheckered} size='sm' fixedWidth />
        <strong>Checkpoint #{event.payload.checkpoint + 1}</strong> was reached {formatTime(new Date(event.timestamp))} with a score of {roundScore(event.payload.score)}.
      </div>
    </div>;
  }

  return null;
}


export default function EvaluationLog({ events }) {
  let hasForfeited = false;

  return <div className='ch-evaluation-log'>
    {events.length > 0 && <h3 className="ch-evaluation-log-label">Submission evaluation timeline</h3>}
    {events.map(event => {
      if (event.type == '_FORFEIT') {
        hasForfeited = true;
      }

      return <EvaluationLogCard key={event.id} event={event} hasForfeited={hasForfeited} />;
    })}
  </div>;
}
