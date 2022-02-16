import { faArrowCircleDown, faSync } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { DoxaError } from 'api/common';
import Card from 'components/Card';
import { useAuth } from 'hooks/useAuth';
import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import ClimateHackAPI from '../api';
import EvaluationLog from '../components/EvaluationLog';
import { roundScore } from '../utils';
import './User.scss';

export default function User({ baseUrl }) {
  const auth = useAuth();
  const { user } = useParams();
  const [profile, setProfile] = useState(null);
  const [score, setScore] = useState(null);
  const [activeAgent, setActiveAgent] = useState(null);
  const [game, setGame] = useState(null);
  const [events, setEvents] = useState(null);
  const [error, setError] = useState(false);
  const [errorMessage, setErrorMessage] = useState(null);
  const [activeTabIndex, setActiveTabIndex] = useState(0);

  const tabs = [
    {
      name: 'Current submission',
      tab: events && <EvaluationLog game={game} events={events} baseUrl={baseUrl} />
    },
    // {
    //   name: 'All submissions',
    //   tab: null
    // },
  ];

  useEffect(async () => {
    if (score !== null) {
      setScore(null);
    }

    try {
      setProfile(await ClimateHackAPI.getUser(user));

      const data = await ClimateHackAPI.getUserScore(user, 'dataset_dapper');
      setScore(data.score || 0.0);
      setActiveAgent(data.agent);
    } catch (e) {
      setError(true);
      if (e instanceof DoxaError) {
        if (e.error_code == 'NO_ACTIVE_AGENT') {
          setErrorMessage('There is no active submission for this user.');
        } else if (e.error_code == 'USER_NOT_FOUND') {
          setErrorMessage('This user does not exist :-(');
        } else {
          setErrorMessage(e.error_message);
        }
      }
      return;
    }

    if (game !== null) {
      setGame(null);
    }

    if (events !== null) {
      setEvents(null);
    }

    try {
      const id = await ClimateHackAPI.getActiveGameId(user);

      if (id !== null && id !== undefined) {
        setGame(await ClimateHackAPI.getGame(id));

        const gameEvents = await ClimateHackAPI.getGameEvents(id, undefined, auth.token);
        if (gameEvents) {
          setEvents(gameEvents || []);
        }
      }

    } catch (e) {
      setErrorMessage(e.error_message);
    }
  }, [user]);

  if (error) {
    return <Card>{errorMessage}</Card>;
  }

  return <>
    <span></span><span></span><span></span><span></span> {/* a fun hack just to get a better outline colour below! */}
    <Card darker className='competitionHeader'>
      <h1>{user}</h1>
      <div className='ch-user-info'>
        {score !== null && <span>
          {roundScore(score / 10000000)}
        </span>}
        {profile && profile._university && profile._university.name && <span>{profile._university.name}</span>}

      </div>


    </Card>

    <div className="ch-user-container">
      <div className="ch-user-tab-selector">
        {tabs.map((tab, i) => <a
          key={i}
          className={activeTabIndex == i ? 'activeTab' : ''}
          onClick={() => setActiveTabIndex(i)}
        >{tab.name}</a>)}
        {auth.user && auth.user.admin && activeAgent && <>
          <a
            href="#"
            onClick={async e => {
              e.preventDefault();
              try {
                await ClimateHackAPI.reactivateAgent(activeAgent, auth.token);
                location.reload();
              } catch {
                console.error(`Could not reactivate agent ${activeAgent}`);
              }
            }}
          >
            <FontAwesomeIcon icon={faSync} size='sm' fixedWidth />
          </a>

          <a
            href="#"
            onClick={async e => {
              e.preventDefault();
              try {
                e.target.style = 'display: none';
                const blob = await ClimateHackAPI.downloadAgent(activeAgent, auth.token);
                e.target.style = 'display: inline-block';

                const link = document.createElement('a');
                link.href = window.URL.createObjectURL(blob);
                link.download = `${activeAgent}.tar.gz`;
                document.body.appendChild(link);
                link.click();
                link.remove();
              } catch {
                console.error(`Could not download agent ${activeAgent}`);
              }
            }}
          >
            <FontAwesomeIcon icon={faArrowCircleDown} size='sm' fixedWidth />
          </a>

        </>}
      </div>

      {tabs[activeTabIndex].tab}
    </div>


  </>;
}
