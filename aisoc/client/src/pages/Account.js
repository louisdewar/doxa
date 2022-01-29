import Button from 'components/Button';
import Card from 'components/Card';
import Container from 'components/Container';
import Navbar from 'components/Navbar';
import { useAuth } from 'hooks/useAuth';
import { Link, Redirect } from 'react-router-dom';


export default function Account({ multipleCompetitionsAllowed }) {
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
        {auth.user.username && <p>
          <Link to={`${multipleCompetitionsAllowed ? '/c/climatehack/' : '/'}user/${auth.user.username}`}>View your latest Climate Hack.AI submission</Link>
        </p>}
      </Card>

      <Link to="/logout">
        <Button>Log out</Button>
      </Link>
    </Container>
  </>;
}
