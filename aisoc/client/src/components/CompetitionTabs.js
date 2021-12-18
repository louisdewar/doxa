import { useState } from 'react';
import Card from './Card';
import './CompetitionTabs.scss';

export default function CompetitionTabs({ tabs }) {
  const [activeTabIndex, setActiveTabIndex] = useState(0);

  return <section className="competitionTabs">
    <div className="competitionTabSelector">
      {tabs.map((tab, i) => <a
        key={i}
        href="#"
        className={activeTabIndex == i ? 'activeTab' : ''}
        onClick={() => setActiveTabIndex(i)}
      >{tab.name}</a>)}
    </div>
    <Card>
      {tabs[activeTabIndex].tab}
    </Card>

  </section>;
}
