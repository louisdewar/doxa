import Card from 'components/Card';
import { useEffect, useState } from 'react';
import { useHistory, useParams } from 'react-router-dom';
import './CompetitionTabs.scss';

export default function CompetitionTabs({ tabs, baseUrl }) {
  const [activeTabIndex, setActiveTabIndex] = useState(0);
  const { tab } = useParams();
  const history = useHistory();

  useEffect(() => {
    if (tab) {
      setActiveTabIndex(tabs.findIndex(x => x.slug == tab));
    }
  }, []);

  useEffect(() => {
    history.push(`${baseUrl}compete/${tabs[activeTabIndex].slug}`);
  }, [activeTabIndex]);

  return <section className="competitionTabs">
    <div className="competitionTabSelector">
      {tabs.map((tab, i) => <a
        key={i}
        className={activeTabIndex == i ? 'activeTab' : ''}
        onClick={() => setActiveTabIndex(i)}
      >{tab.name}</a>)}
    </div>
    <Card>
      {tabs[activeTabIndex].tab}
    </Card>

  </section>;
}
