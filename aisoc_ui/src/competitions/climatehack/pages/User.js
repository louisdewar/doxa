import Matches from 'components/Matches';
import Navbar from 'components/Navbar';
import { useParams } from 'react-router-dom';

export default function User({ baseUrl }) {
  const { user } = useParams();

  return <>
    <Navbar competition="climatehack" competitionName="Climate Hack" />
    <div className='container'>
      <header className='competitionHeader'>
        <h1>{user}</h1>
        <h2>
          0 points
        </h2>
      </header>
      <section className="competitionTabs">
        <div className="competitionTab">
          <h2>Matches</h2>
          <Matches baseUrl={baseUrl} />
        </div>
      </section>
    </div>
  </>;
}
