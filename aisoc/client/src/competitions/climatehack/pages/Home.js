import CompetitionHeader from 'components/CompetitionHeader';
import CompetitionTabs from 'components/CompetitionTabs';
import GettingStartedTab from '../components/tabs/GettingStartedTab';
import LeaderboardTab from '../components/tabs/LeaderboardTab';
import OverviewTab from '../components/tabs/OverviewTab';
import RulesTab from '../components/tabs/RulesTab';
import SubmissionGuideTab from '../components/tabs/SubmissionGuideTab';
import './Home.scss';

export default function Home({ baseUrl }) {
  // Overview Data Code Leaderboard Rules

  const tabs = [
    {
      name: 'OVERVIEW',
      tab: <OverviewTab />
    },
    {
      name: 'DATA',
      tab: <div>
        <h2>Data</h2>
      </div>
    },
    {
      name: 'GETTING STARTED',
      tab: <GettingStartedTab />
    },
    {
      name: 'SUBMISSION GUIDE',
      tab: <SubmissionGuideTab />
    },
    {
      name: 'LEADERBOARD',
      tab: <LeaderboardTab baseUrl={baseUrl} />
    },
    {
      name: 'RULES',
      tab: <RulesTab />
    },
  ];

  return <>
    <CompetitionHeader
      competitionName="Climate Hack"
      description="Climate Hack is an alliance between the artificial intelligence societies of some of the world's best universities in the fight against climate change. Your challenge is to beat the current best nowcasting techniques for UK satellite imagery. Through improved predictions of cloud coverage and hence solar photovoltaic power output, your model could help National Grid minimise the use of idling natural gas turbines, saving potentially up to 100 kilotonnes of carbon emissions each year."
    />
    <CompetitionTabs tabs={tabs} />
  </>;
}
