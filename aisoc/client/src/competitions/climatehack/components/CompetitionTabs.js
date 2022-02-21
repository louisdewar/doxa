import Card from 'components/Card';
import { useEffect, useRef, useState } from 'react';
import { useHistory, useLocation, useParams } from 'react-router-dom';
import './CompetitionTabs.scss';

export default function CompetitionTabs({ tabs, baseUrl }) {
  const [activeTabIndex, setActiveTabIndex] = useState(0);
  const { tab } = useParams();
  const mountedRef = useRef(false);
  const location = useLocation();
  const history = useHistory();

  useEffect(() => {
    if (tab) {
      setActiveTabIndex(tabs.findIndex(x => x.slug == tab));
    } else if (location.pathname.endsWith('/compete') || location.pathname.endsWith('/compete/')) {
      history.push(`${baseUrl}compete/${tabs[activeTabIndex].slug}`);
    }
  }, []);

  useEffect(() => {
    if (mountedRef.current) {
      history.push(`${baseUrl}compete/${tabs[activeTabIndex].slug}`);
    } else {
      mountedRef.current = true;
    }
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
