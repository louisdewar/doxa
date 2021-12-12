import Card from './Card';
import './CompetitionHeader.scss';

export default function CompetitionHeader({ competitionName, description, participantCount }) {
  return <Card darker className='competitionHeader'>
    <h1>{competitionName}</h1>
    <p>
      {description}
    </p>
    <p>
      {participantCount} participating
    </p>
  </Card>;
}
