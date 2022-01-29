import ClimateHackAPI from 'competitions/climatehack/api';
import { useAuth } from 'hooks/useAuth';
import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import Leaderboard from '../Leaderboard';
import './LeaderboardTab.scss';



export default function LeaderboardTab({ baseUrl }) {
  const [leaderboard, setLeaderboard] = useState(null);
  const auth = useAuth();

  useEffect(() => {
    ClimateHackAPI.getLeaderboard('dataset_dapper').then(data => {
      setLeaderboard(data);
    }).catch(err => {
      console.error(err);
    });
  }, []);

  const tabs = [
    {
      name: 'ROUND 1',
      tab: leaderboard && <Leaderboard baseUrl={baseUrl} leaderboard={leaderboard} />
    },
    {
      name: 'ROUND 2',
      tab: <div>
        Round 2 of Climate Hack.AI has not started yet &mdash; check back here on 24th March!
      </div>
    },
  ];

  const [activeTabIndex, setActiveTabIndex] = useState(0);

  return <div className="ch-tab ch-leaderboard-tab">
    <h2>Leaderboard</h2>

    <div className="ch-leaderboard-tab-selector">
      {tabs.map((tab, i) => <a
        key={i}
        className={activeTabIndex == i ? 'activeTab' : ''}
        onClick={() => setActiveTabIndex(i)}
      >{tab.name}</a>)}
      {auth.isLoggedIn() && <Link to={`${baseUrl}user/${auth.user.username}`}>Your submission</Link>}
    </div>

    {tabs[activeTabIndex].tab}
  </div>;
}
