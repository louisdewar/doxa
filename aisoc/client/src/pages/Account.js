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
      </Card>

      <Link to="/logout">
        <Button>Log out</Button>
      </Link>
    </Container>
  </>;
}
