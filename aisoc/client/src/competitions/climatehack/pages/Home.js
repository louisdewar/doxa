import CompetitionHeader from 'components/CompetitionHeader';
import CompetitionTabs from 'components/CompetitionTabs';
import Leaderboard from 'components/Leaderboard';

export default function Home({ baseUrl }) {
  const leaderboard = [{ 'agent': 'a210675ab354f33d488f', 'score': 41, 'username': 'testaccount' }, { 'agent': 'c854ab2e9ce7f8c3dcdd', 'score': 39, 'username': 'louisdewardt' }];

  const tabs = [
    {
      name: 'OVERVIEW',
      tab: <div>
        <h2>Overview</h2>
        <p>
          Cupcake ipsum dolor sit amet I love macaroon dessert I love. Gingerbread wafer wafer I love oat cake jelly ice cream. Sesame snaps topping I love candy danish I love sesame snaps I love tootsie roll. Candy canes chocolate cake jelly-o pudding soufflé lollipop icing.
        </p>
        <p>
          Gingerbread carrot cake jujubes croissant icing sweet. Fruitcake brownie cookie I love sesame snaps bear claw cotton candy lemon drops sugar plum. Lollipop tart brownie pudding oat cake halvah cake carrot cake caramels. Carrot cake I love I love pastry cake.
        </p>
        <p>
          Cupcake danish soufflé marzipan I love jelly-o. Jelly beans pudding pastry chocolate bar marshmallow toffee chocolate cake cupcake caramels. I love marzipan chocolate chocolate pastry ice cream donut cake I love.
        </p>
        <p>
          Dessert dragée cheesecake biscuit marshmallow cake. Dessert macaroon I love chupa chups biscuit jelly oat cake sesame snaps marshmallow. I love marshmallow shortbread biscuit jelly-o. Danish brownie macaroon topping donut cake caramels cake.
        </p>
      </div>
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
      tab: <div>
        <h2>Leaderboard</h2>
        <Leaderboard baseUrl={baseUrl} leaderboard={leaderboard} />
      </div>
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
      competitionName="Climate Hack"
      description="Climate Hack is an alliance between the artificial intelligence societies of some of the world's best universities in the fight against climate change. Your challenge is to beat current UK cloud coverage forecasts so that predictions of future solar photovoltaic power production may be improved. This could allow National Grid to minimise the use of idling natural gas turbines, saving potentially up to 100 kilotonnes in carbon emissions per year in the process."
    />
    <CompetitionTabs tabs={tabs} />
  </>;
}
