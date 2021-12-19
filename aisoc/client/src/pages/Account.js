import { COMPETITIONS } from 'competitions';
import Button from 'components/Button';
import Card from 'components/Card';
import Container from 'components/Container';
import Navbar from 'components/Navbar';
import { useAuth } from 'hooks/useAuth';
import { Link, Redirect } from 'react-router-dom';


export default function Account() {
  const auth = useAuth();

  if (!auth.isLoggedIn()) { return <Redirect to='/' />; }
  if (!auth.user) { return <></>; }

  return <>
    <Navbar />
    <Container>
      <Card>
        {auth.user.username && <h1>Hi, {auth.user.username}!</h1>}
        {auth.user.admin && <p>
          <strong>You are an admin.</strong>
        </p>}
        {auth.user.competitions && <p>
          <strong>Competition enrolments</strong>: {
            auth.user.competitions
              .map(competition => <Link to={`/c/${competition}/`} key={competition}>{(COMPETITIONS[competition] ?? { name: competition }).name}</Link>)
              .reduce(((a, b) => [a, ', ', b]))
          }
        </p>}
      </Card>

      <Link to="/logout">
        <Button>Log out</Button>
      </Link>
    </Container>
  </>;
}
