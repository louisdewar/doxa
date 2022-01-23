import { faClock } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import './OngoingCard.scss';

export default function OngoingCard({ started }) {
  return (
    <div className="game-card">
      <div className="game-card-body ongoing">
        <div className="game-card-ongoing-container">
          <div className="ongoing-icon"><FontAwesomeIcon icon={faClock} fixedWidth /></div>
          <div className="ongoing-message">
            <p>
              This match {started ? 'is ongoing' : 'has been queued'}, so more games may appear here soon.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
