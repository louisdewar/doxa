import { faClock, faFlagCheckered, faHourglassEnd, faHourglassStart } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { formatTime } from 'utils/time';
import { roundScore } from '../utils';
import './EvaluationLog.scss';



function EvaluationLogCard({ event }) {
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
  } else if (event.type.startsWith('checkpoint')) {
    return <div className='ch-evaluation-log-card'>
      <div className='ch-evaluation-log-card-body'>
        <FontAwesomeIcon icon={faFlagCheckered} size='sm' fixedWidth />
        <strong>Checkpoint #{event.payload.checkpoint + 1}</strong> was reached {formatTime(new Date(event.timestamp))} with score {roundScore(event.payload.score)}.
      </div>
    </div>;
  }
}


export default function EvaluationLog({ events }) {
  return <div className='ch-evaluation-log'>
    {events.length > 0 && <h3 className="ch-evaluation-log-label">Submission evaluation timeline</h3>}
    {events.map(event => <EvaluationLogCard key={event.id} event={event} />)}
  </div>;
}
