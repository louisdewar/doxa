import { faExclamationTriangle } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import PlayerLink from '../../components/PlayerLink/PlayerLink';
import './ErrorCard.scss';


const PLAYER_CLASS = ['main', 'opposing'];


export default function ErrorCard({ forfeit, error, players, baseUrl, canShowForfeitedError }) {
  // Errors not related to game forfeits represent internal server errors.
  return <>
    {/* The forfeit error card only shows when there is no game card to attach the forfeit error message to. */}
    {forfeit && canShowForfeitedError && <div className='game-card'>
      <div className={`game-card-body error ${forfeit && forfeit.payload && ['lost', 'won'][forfeit.payload.agent]}`}>
        <div className="game-card-error-header">
          <div className="error-icon"><FontAwesomeIcon icon={faExclamationTriangle} fixedWidth /></div>
          <div className="error-message">
            <p>
              <PlayerLink username={players[forfeit.payload.agent].username} baseUrl={baseUrl} playerClass={PLAYER_CLASS[forfeit.payload.agent]} />&apos;s agent forfeited the game, so <PlayerLink username={players[forfeit.payload.agent === 0 ? 1 : 0].username} baseUrl={baseUrl} playerClass={PLAYER_CLASS[forfeit.payload.agent === 0 ? 1 : 0]} /> wins
              the remaining {forfeit.payload.remaining ?? 0} {forfeit.payload.remaining ?? 0 > 1 ? 'games' : 'game'} in the match by default.
            </p>
          </div>
        </div>
      </div>
    </div>}

    {error && !forfeit && <div className='game-card'>
      <div className={`game-card-body error ${forfeit && forfeit.payload && ['lost', 'won'][forfeit.payload.agent]}`}>
        <div className="game-card-error-header">
          <div className="error-icon"><FontAwesomeIcon icon={faExclamationTriangle} fixedWidth /></div>
          <div className="error-message">
            <p>
              An internal server error occured while running this match, so it could not continue.
              The match will be rescheduled if either <PlayerLink username={players[0].username} baseUrl={baseUrl} playerClass={'main'} /> or <PlayerLink username={players[1].username} baseUrl={baseUrl} playerClass={'opposing'} /> reupload their agent.
            </p>
          </div>
        </div>
      </div>
    </div>}

    {((error && error.payload) || (forfeit && forfeit.payload && (forfeit.payload.reason || forfeit.payload.stderr))) && <div className='game-card'>
      <div className={'game-card-body error error-output'}>
        {forfeit && forfeit.payload && <>
          {forfeit.payload.reason && <>
            <p className="logs-message">Forfeit reason:</p>
            <pre className="logs">{forfeit.payload.reason}</pre>
          </>}

          {forfeit.payload.stderr && <>
            <p className="logs-message">You have permission to view the <code>stderr</code> of <PlayerLink username={players[forfeit.payload.agent].username} baseUrl={baseUrl} playerClass={PLAYER_CLASS[forfeit.payload.agent]} />&apos;s agent (max 100 KB):</p>
            <pre className="logs">{forfeit.payload.stderr}</pre>
          </>}
        </>}

        {error && error.payload && <>
          {error.payload.error && <>
            <p className="logs-message">Error message:</p>
            <pre className="logs">
              {error.payload.error}
            </pre>
          </>}

          {error.payload.debug && <>
            <p className="logs-message">Debug error message:</p>
            <pre className="logs">
              {error.payload.debug}
            </pre>
          </>}

          {error.payload.vm_logs && <>
            <p className="logs-message">Virtual machine logs for <PlayerLink username={players[0].username} baseUrl={baseUrl} playerClass={'main'} />&apos;s agent:</p>
            <pre className="logs">
              {error.payload.vm_logs[0] ?? <em>No log content to display.</em>}&nbsp;
            </pre>

            <p className="logs-message">Virtual machine logs for <PlayerLink username={players[1].username} baseUrl={baseUrl} playerClass={'opposing'} />&apos;s agent:</p>
            <pre className="logs">
              {error.payload.vm_logs[1] ?? <em>No log content to display.</em>}&nbsp;
            </pre>
          </>}
        </>}
      </div>
    </div>}
  </>;
}
