import './CompetitionHeader.scss';

export default function CompetitionHeader({ competitionName, description, participantCount }) {
  return <header className='competitionHeader'>
    <h1>{competitionName}</h1>
    <p>
      {description}
    </p>
    <p>
      {participantCount} participating
    </p>
  </header>;
}
