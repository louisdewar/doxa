import { useState } from 'react';
import Leaderboard from '../Leaderboard';
import './LeaderboardTab.scss';


export default function LeaderboardTab({ baseUrl }) {
  const leaderboard = [{ 'score': 41, 'username': 'testaccount', 'university': 'UCL' }, { 'score': 39, 'username': 'louisdewardt', 'university': 'UCL' }];

  const tabs = [
    {
      name: 'ROUND 1',
      tab: <Leaderboard baseUrl={baseUrl} leaderboard={leaderboard} />
    },
    {
      name: 'ROUND 2',
      tab: <div>
        Round 2 of Climate Hack has not started yet &mdash; check back here on 24th March!
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
    </div>

    {tabs[activeTabIndex].tab}
  </div>;
}
