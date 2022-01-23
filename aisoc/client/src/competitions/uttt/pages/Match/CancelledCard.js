import { faBan } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import './CancelledCard.scss';

export default function CancelledCard() {
  return (
    <div className="game-card">
      <div className="game-card-body cancelled">
        <div className="game-card-cancelled-container">
          <div className="cancelled-icon"><FontAwesomeIcon icon={faBan} fixedWidth /></div>
          <div className="cancelled-message">
            <p>
              This match was cancelled, so there will not be any more game events.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
