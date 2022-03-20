import CompetitionHeader from 'components/CompetitionHeader';
import CompetitionTabs from '../components/CompetitionTabs';
import FaqTab from '../components/tabs/FaqTab';
import GettingStartedGuideTab from '../components/tabs/GettingStartedGuideTab';
import LeaderboardTab from '../components/tabs/LeaderboardTab';
import OverviewTab from '../components/tabs/OverviewTab';
import ResourcesTab from '../components/tabs/ResourcesTab';
import './Home.scss';

export default function Home({ baseUrl }) {
  // Overview Data Code Leaderboard Rules

  const tabs = [
    {
      name: 'OVERVIEW',
      tab: <OverviewTab baseUrl={baseUrl} />,
      slug: 'overview'
    },
    {
      name: 'RESOURCES',
      tab: <ResourcesTab />,
      slug: 'resources'
    },
    {
      name: 'GETTING STARTED',
      tab: <GettingStartedGuideTab />,
      slug: 'getting-started'
    },
    {
      name: 'LEADERBOARD',
      tab: <LeaderboardTab baseUrl={baseUrl} />,
      slug: 'leaderboard'
    },
    // {
    //   name: 'RULES',
    //   tab: <RulesTab />,
    //   slug: 'rules'
    // },
    {
      name: 'FAQ',
      tab: <FaqTab />,
      slug: 'faq'
    },
  ];

  return <>
    <CompetitionHeader
      competitionName="Climate Hack.AI"
      description={<>
        {'Climate Hack.AI is an alliance between the artificial intelligence societies of some of the world\'s best universities in the fight against climate change. Your challenge is to beat the current best nowcasting techniques for UK satellite imagery.'}
        <br /><br />
        {'By helping to improve solar photovoltaic power output predictions, your model could help the National Grid Electricity System Operator minimise the use of standby natural gas turbines, potentially saving up to 100 kilotonnes of carbon emissions a year.'}
        <br /><br />
        Make sure to join our <a href="https://discord.gg/HTTQ8AFjJp">Discord</a> and follow us on <a href="https://linktr.ee/climatehack.ai">social media</a> to receive updates throughout the competition.
      </>}
    />
    <CompetitionTabs tabs={tabs} baseUrl={baseUrl} />
  </>;
}
