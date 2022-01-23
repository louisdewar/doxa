import { faExclamationTriangle } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import PlayerLink from '../../components/PlayerLink/PlayerLink';
import './ErrorCard.scss';


const PLAYER_CLASS = ['main', 'opposing'];


<<<<<<< HEAD
export default function ErrorCard({ forfeit, error, players, baseUrl }) {
  let errorMessage;
  let extraInfo;

  if (forfeit && forfeit.payload) {
    const forfeiter = forfeit.payload.agent;
    const other = forfeiter === 0 ? 1 : 0;
    const stderr = forfeit.payload.stderr;
    const remaining = forfeit.payload.remaining ?? 0;

    errorMessage = (
      <div className='error-message'>
        <p><PlayerLink username={players[forfeiter].username} baseUrl={baseUrl} playerClass={PLAYER_CLASS[forfeiter]} />&apos;s agent forfeit the match!</p>
        <p>
          As a result, <PlayerLink username={players[other].username} baseUrl={baseUrl} playerClass={PLAYER_CLASS[other]} /> wins
          the remaining {remaining} {remaining > 1 ? 'games' : 'game'} by default.
        </p>
      </div>
    );

    if (stderr) {
      extraInfo = (
        <>
          <p className="logs-message">You have permission to view the <code>stderr</code> of <PlayerLink username={players[forfeiter].username} baseUrl={baseUrl} playerClass={PLAYER_CLASS[forfeiter]} />&apos;s agent (max 50MiB):</p>
          <pre className="logs">{stderr}</pre>
        </>
      );
    }
  }

  if (error) {
    // If the error was not a forfeit it represents an internal error
    if (!forfeit) {
      errorMessage = (
        <>
          <p>An internal error occured when running this match that meant it couldn&apos;t continue.</p>
          <p>
            The match can be re-rescheduled by either <PlayerLink username={players[0].username} baseUrl={baseUrl} playerClass={'main'} /> {' '}
            or <PlayerLink username={players[1].username} baseUrl={baseUrl} playerClass={'opposing'} /> re-uploading their agent.
          </p>
        </>
      );
    }

    if (error.payload) {
      if (error.payload.error) {
        extraInfo = <>
          {extraInfo}
          <p className="logs-message">Error message:</p>
          <pre className="logs">
            {error.payload.error}
          </pre>
        </>;
      }

      if (error.payload.debug) {
        extraInfo = <>
          {extraInfo}
          <p className="logs-message">Debug error message:</p>
          <pre className="logs">
            {error.payload.debug}
          </pre>
        </>;
      }

      if (error.payload.vm_logs) {
        const vm_logs = error.payload.vm_logs;
        extraInfo = <>
          {extraInfo}
          <p className="logs-message">Virtual machine logs for <PlayerLink username={players[0].username} baseUrl={baseUrl} playerClass={'main'} />:</p>
          <pre className="logs">
            {vm_logs[0]}&nbsp;
          </pre>

          <p className="logs-message">Virtual machine logs for <PlayerLink username={players[1].username} baseUrl={baseUrl} playerClass={'opposing'} />:</p>
          <pre className="logs">
            {vm_logs[1]}&nbsp;
          </pre>
        </>;
      }
    }
  }

  return <>
    <div className={`game-card error ${['lost', 'won'][forfeit.payload.agent]}`}>
      <div className="game-card-error-header">
        <div className="error-icon"><FontAwesomeIcon icon={faExclamationTriangle} fixedWidth /></div>
        {errorMessage}
      </div>
    </div>
    {extraInfo && <div className={'game-card error error-output'}>
      {extraInfo}
=======
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
            <p className="logs-message">You have permission to view the <code>stderr</code> of <PlayerLink username={players[forfeit.payload.agent].username} baseUrl={baseUrl} playerClass={PLAYER_CLASS[forfeit.payload.agent]} />&apos;s agent (max 50MiB):</p>
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
>>>>>>> origin/master
    </div>}
  </>;
}
