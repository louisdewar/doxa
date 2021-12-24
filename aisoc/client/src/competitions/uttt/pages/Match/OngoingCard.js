import { faClock } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import './OngoingCard.scss';

export default function OngoingCard({ started }) {
  return (
    <div className="game-card ongoing">
      <div className="game-card-ongoing-container">
        <div className="ongoing-icon"><FontAwesomeIcon icon={faClock} fixedWidth /></div>
        <div className="ongoing-message">
          <p>
            This match {started ? 'is ongoing' : 'has been queued'}, so more games may appear here in the future.
          </p>
        </div>
      </div>
    </div>
  );
}
