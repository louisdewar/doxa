import ClimateHackAPI from 'competitions/climatehack/api';
import { useAuth } from 'hooks/useAuth';
import { useEffect, useState } from 'react';
import { Link, useHistory, useParams } from 'react-router-dom';
import Leaderboard from '../Leaderboard';
import UniversityLeaderboard from '../UniversityLeaderboard';
import './LeaderboardTab.scss';



export default function LeaderboardTab({ baseUrl }) {
  const [activeTabIndex, setActiveTabIndex] = useState(0);
  const [leaderboard, setLeaderboard] = useState(null);
  const auth = useAuth();

  const tabs = [
    {
      name: 'PARTICIPANTS',
      tab: leaderboard && <Leaderboard baseUrl={baseUrl} leaderboard={leaderboard} />,
      slug: 'participants'
    },
    {
      name: 'UNIVERSITIES',
      tab: leaderboard && <UniversityLeaderboard baseUrl={baseUrl} leaderboard={leaderboard} />,
      slug: 'universities'
    },
    // {
    //   name: 'ROUND 2',
    //   tab: <div>
    //     Round 2 of Climate Hack.AI has not started yet &mdash; check back here on 24th March!
    //   </div>
    // },
  ];

  const { subtab } = useParams();
  const history = useHistory();

  useEffect(() => {
    if (subtab) {
      setActiveTabIndex(tabs.findIndex(x => x.slug == subtab));
    }
  }, []);

  useEffect(() => {
    history.push(`${baseUrl}compete/leaderboard/${tabs[activeTabIndex].slug}`);
  }, [activeTabIndex]);

  useEffect(() => {
    ClimateHackAPI.getLeaderboard('dataset_dapper').then(data => {
      setLeaderboard(data);
    }).catch(err => {
      console.error(err);
    });
  }, []);





  return <div className="ch-tab ch-leaderboard-tab">
    <h2>Leaderboard</h2>

    <div className="ch-leaderboard-tab-selector">
      {tabs.map((tab, i) => <a
        key={i}
        className={activeTabIndex == i ? 'activeTab' : ''}
        onClick={() => setActiveTabIndex(i)}
      >{tab.name}</a>)}
      {auth.isLoggedIn() && auth.user && auth.user.username && <Link to={`${baseUrl}user/${auth.user.username}`}>YOUR SUBMISSION</Link>}
    </div>

    {tabs[activeTabIndex].tab}
  </div>;
}
