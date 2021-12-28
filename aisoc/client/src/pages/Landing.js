import Card from 'components/Card';
import Container from 'components/Container';
import Navbar from 'components/Navbar';
import { Link } from 'react-router-dom';
import './Landing.scss';



export default function Landing({ competitions }) {
  return <>
    <Navbar />
    <Container>
      {location.hash && location.hash.length > 1 && location.hash.includes('invite-success') && <Card darker>
        <pre>
          Your account was created successfully! You may now log in.
        </pre>
      </Card>}
      {location.hash && location.hash.length > 1 && location.hash.includes('login-success') && <Card darker>
        <pre>
          You were successfully logged in!
        </pre>
      </Card>}
      <Card darker>
        <h1 style={{
          'textUnderlineOffset': '0.65rem',
          'textDecoration': 'underline',
          'fontSize': '3rem',
          'marginTop': 0,
          'marginBottom': '0.65rem',
        }}>Doxa</h1>
      </Card>
      <div className='open-competitions-label'>
        OPEN COMPETITIONS
      </div>
      {Object.keys(competitions).map(competition => <Card key={competition} className='competition-summary-card'>
        <h2>
          <Link
            to={`/c/${competition}/`}
            className='competition-summary-link'
          >{competitions[competition].name}</Link>
        </h2>
      </Card>)}
    </Container>
  </>;
}
