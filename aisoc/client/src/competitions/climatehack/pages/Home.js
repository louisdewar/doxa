import CompetitionHeader from 'components/CompetitionHeader';
import CompetitionTabs from 'components/CompetitionTabs';
import FaqTab from '../components/tabs/FaqTab';
import GettingStartedGuideTab from '../components/tabs/GettingStartedGuideTab';
import LeaderboardTab from '../components/tabs/LeaderboardTab';
import OverviewTab from '../components/tabs/OverviewTab';
import ResourcesTab from '../components/tabs/ResourcesTab';
import RulesTab from '../components/tabs/RulesTab';
import './Home.scss';

export default function Home({ baseUrl }) {
  // Overview Data Code Leaderboard Rules

  const tabs = [
    {
      name: 'OVERVIEW',
      tab: <OverviewTab baseUrl={baseUrl} />
    },
    {
      name: 'RESOURCES',
      tab: <ResourcesTab />
    },
    {
      name: 'GETTING STARTED GUIDE',
      tab: <GettingStartedGuideTab />
    },
    {
      name: 'LEADERBOARD',
      tab: <LeaderboardTab baseUrl={baseUrl} />
    },
    {
      name: 'RULES',
      tab: <RulesTab />
    },
    {
      name: 'FAQ',
      tab: <FaqTab />
    },
  ];

  return <>
    <CompetitionHeader
      competitionName="Climate Hack"
      description={<>
        {'Climate Hack is an alliance between the artificial intelligence societies of some of the world\'s best universities in the fight against climate change. Your challenge is to beat the current best nowcasting techniques for UK satellite imagery.'}
        <br /><br />
        {'By helping to improve solar photovoltaic power output predictions, your model could help the National Grid Electricity System Operator minimise the use of standby natural gas turbines, potentially saving up to 100 kilotonnes of carbon emissions a year.'}
      </>}
    />
    <CompetitionTabs tabs={tabs} />
  </>;
}
