import { useEffect, useState } from 'react';
import { useLocation } from 'react-router-dom';
import Card from './Card';
import './CompetitionTabs.scss';

export default function CompetitionTabs({ tabs }) {
  const location = useLocation();
  const [activeTabIndex, setActiveTabIndex] = useState(0);

  useEffect(() => {
    if (location.hash && location.hash.length > 1) {
      const hash = +location.hash.replace('#', '');
      if (hash && 0 < hash < tabs.length) {
        setActiveTabIndex(hash);
      }
    }
  }, [location.hash]);

  return <section className="competitionTabs">
    <div className="competitionTabSelector">
      {tabs.map((tab, i) => <a
        key={i}
        href={`#${i}`}
        className={activeTabIndex == i ? 'activeTab' : ''}
        onClick={() => setActiveTabIndex(i)}
      >{tab.name}</a>)}
    </div>
    <Card>
      {tabs[activeTabIndex].tab}
    </Card>

  </section>;
}
