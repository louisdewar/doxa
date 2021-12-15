import CompetitionHeader from 'components/CompetitionHeader';
import CompetitionTabs from 'components/CompetitionTabs';
import LeaderboardTab from '../tabs/LeaderboardTab';
import OverviewTab from '../tabs/OverviewTab';

export default function Home({ baseUrl }) {


  const tabs = [
    {
      name: 'OVERVIEW',
      tab: <OverviewTab />
    },
    {
      name: 'RULES',
      tab: <div>
        <h2>Rules</h2>
      </div>
    },
    {
      name: 'DATA',
      tab: <div>
        <h2>Data</h2>
      </div>
    },
    {
      name: 'CODE',
      tab: <div>
        <h2>Code</h2>
      </div>
    },
    {
      name: 'LEADERBOARD',
      tab: <LeaderboardTab baseUrl={baseUrl} />
    },
    {
      name: 'SUBMISSION GUIDE',
      tab: <div>
        <h2>Submission Guide</h2>
      </div>
    },
  ];


  return <>
    <CompetitionHeader
      competitionName="Ultimate Tic-Tac-Toe"
      description="It is what it is."
    />
    <CompetitionTabs tabs={tabs} />
  </>;
}
