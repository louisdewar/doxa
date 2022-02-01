import { faBan, faCheckCircle, faCircleNotch, faClock, faExclamationTriangle, faFlagCheckered, faHourglassEnd, faHourglassStart } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { formatTime } from 'utils/time';
import { roundScore } from '../utils';
import './EvaluationLog.scss';



function EvaluationLogCard({ event, hasForfeited }) {
  const timestamp = new Date(event.timestamp);

  if (event.type == '_START' || event.type == '_END') {
    return <div className='ch-evaluation-card ch-evaluation-endpoint'>
      <div className='ch-evaluation-card-body'>
        <FontAwesomeIcon icon={event.type == '_START' ? faHourglassStart : faHourglassEnd} size='sm' fixedWidth />
        <div className='ch-evaluation-card-text' title={timestamp.toLocaleString()}>
          {event.type == '_START' ? 'Evaluation started' : 'Evaluation terminated'} {formatTime(timestamp)}.
        </div>
      </div>
    </div>;
  } else if (event.type == '_QUEUED') {
    return <div className='ch-evaluation-card ch-evaluation-endpoint'>
      <div className='ch-evaluation-card-body'>
        {event.payload.started_at
          ? <FontAwesomeIcon icon={faCheckCircle} size='sm' fixedWidth />
          : <FontAwesomeIcon icon={faCircleNotch} size='sm' fixedWidth spin />}
        <div className='ch-evaluation-card-text' title={event.payload.queued_at.toLocaleString()}>
          This submission was queued {formatTime(event.payload.queued_at)}.
        </div>
      </div>
    </div>;
  } else if (event.type == 'final') {
    return <div className='ch-evaluation-card'>
      <div className='ch-evaluation-card-body'>
        <FontAwesomeIcon icon={faClock} size='sm' fixedWidth />
        <div className='ch-evaluation-card-text'>
          The final score for this submission is {roundScore(event.payload.score)}.
        </div>
      </div>
    </div>;
  } else if (event.type == '_FORFEIT') {
    return <>
      <div className='ch-evaluation-card ch-evaluation-error'>
        <div className={`ch-evaluation-card-body ${event.payload ? 'ch-evaluation-error-available' : ''}`}>
          <FontAwesomeIcon icon={faExclamationTriangle} size='sm' fixedWidth />
          <div className='ch-evaluation-card-text' title={timestamp && timestamp.toLocaleString()}>
            An error occurred while running this submission&apos;s code.
          </div>
        </div>

        {event.payload && (event.payload.reason || event.payload.stderr) && <div className='ch-evaluation-card-error-info'>
          {event.payload.reason && <>
            <p className="ch-evaluation-card-error-info-logs-label">The agent forfeited for the following reason:</p>
            <pre className="ch-evaluation-card-error-info-logs">{event.payload.reason}</pre>
          </>}

          {event.payload.stderr && <>
            <p className="ch-evaluation-card-error-info-logs-label">You have permission to view the agent&apos;s <code>stderr</code> (max 50 MiB):</p>
            <pre className="ch-evaluation-card-error-info-logs">{event.payload.stderr}</pre>
          </>}
        </div>}
      </div>
    </>;
  } else if (event.type == '_ERROR' && (!hasForfeited || event.payload)) {
    return <>
      <div className='ch-evaluation-card ch-evaluation-error'>
        <div className={`ch-evaluation-card-body ${event.payload ? 'ch-evaluation-error-available' : ''}`}>
          <FontAwesomeIcon icon={faExclamationTriangle} size='sm' fixedWidth />
          <div className='ch-evaluation-card-text' title={timestamp && timestamp.toLocaleString()}>
            An error occurred during the evaluation of this submission.
          </div>
        </div>

        {event.payload && (event.payload.error || event.payload.debug || event.payload.vm_logs) && <div className='ch-evaluation-card-error-info'>
          {event.payload.error && <>
            <p className="ch-evaluation-card-error-info-logs-label">The agent error message was the following:</p>
            <pre className="ch-evaluation-card-error-info-logs">{event.payload.error}</pre>
          </>}

          {event.payload.debug && <>
            <p className="ch-evaluation-card-error-info-logs-label">The agent debug error message was the following:</p>
            <pre className="ch-evaluation-card-error-info-logs">{event.payload.debug}</pre>
          </>}

          {event.payload.vm_logs && <>
            <p className="ch-evaluation-card-error-info-logs-label">The agent virtual machine logs show the following:</p>
            <pre className="ch-evaluation-card-error-info-logs">{event.payload.vm_logs}</pre>
          </>}
        </div>}
      </div>
    </>;
  } else if (event.type == '_CANCELLED') {
    return <div className='ch-evaluation-card ch-evaluation-cancelled'>
      <div className='ch-evaluation-card-body'>
        <FontAwesomeIcon icon={faBan} size='sm' fixedWidth />
        <div className='ch-evaluation-card-text' title={timestamp && timestamp.toLocaleString()}>
          The evaluation of this submission was cancelled.
        </div>
      </div>
    </div>;
  } else if (event.type.startsWith('checkpoint')) {
    return <div className='ch-evaluation-card'>
      <div className='ch-evaluation-card-body'>
        <FontAwesomeIcon icon={faFlagCheckered} size='sm' fixedWidth />
        <div className='ch-evaluation-card-text' title={timestamp && timestamp.toLocaleString()}>
          <strong>Checkpoint #{event.payload.checkpoint + 1}</strong> was reached {formatTime(timestamp)} with a score of {roundScore(event.payload.score)}.
        </div>
        {event.payload.images && <img src={`data:image/png;base64,${event.payload.images[0]}`} alt="Model output image" />}
      </div>
    </div>;
  }

  return null;
}


export default function EvaluationLog({ game, events }) {
  let hasForfeited = false;

  return <div className='ch-evaluation'>
    {events.length > 0 && <h3 className="ch-evaluation-label">Submission evaluation timeline</h3>}
    {game && game.queued_at && <EvaluationLogCard event={{
      type: '_QUEUED',
      payload: {
        queued_at: game.queued_at,
        started_at: game.started_at,
      }
    }} hasForfeited={hasForfeited} />}
    {events.map(event => {
      if (event.type == '_FORFEIT') {
        hasForfeited = true;
      }

      return <EvaluationLogCard key={event.id} event={event} hasForfeited={hasForfeited} />;
    })}
  </div>;
}
