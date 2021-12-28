import CompetitionHeader from 'components/CompetitionHeader';
import CompetitionTabs from 'components/CompetitionTabs';
import LeaderboardTab from './LeaderboardTab';
import OverviewTab from './OverviewTab';
import RulesTab from './RulesTab';
import SubmissionGuideTab from './SubmissionGuideTab';

export default function Home({ baseUrl }) {
  const tabs = [
    {
      name: 'OVERVIEW',
      tab: <OverviewTab />
    },
    {
      name: 'RULES',
      tab: <RulesTab />
    },
    {
      name: 'LEADERBOARD',
      tab: <LeaderboardTab baseUrl={baseUrl} />
    },
    {
      name: 'SUBMISSION GUIDE',
      tab: <SubmissionGuideTab />
    },
  ];

  return <>
    <CompetitionHeader
      competitionName="Ultimate Tic-Tac-Toe"
      description="The aim of the game is to win a tic-tac-toe grid of tac-tac-toe grids!"
    />
    <CompetitionTabs tabs={tabs} />
  </>;
}
