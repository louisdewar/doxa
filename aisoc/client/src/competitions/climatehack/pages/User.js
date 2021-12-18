import Card from 'components/Card';
import Matches from 'components/Matches';
import { useParams } from 'react-router-dom';

export default function User({ baseUrl }) {
  const { user } = useParams();

  return <>
    <Card darker className='competitionHeader'>
      <h1>{user}</h1>
      <h2>
        0 points
      </h2>
    </Card>
    <Card>
      <h2>Submissions</h2>
      <Matches baseUrl={baseUrl} />
    </Card>
  </>;
}
