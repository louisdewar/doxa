import Button from 'components/Button';
import Card from 'components/Card';
import Container from 'components/Container';
import Navbar from 'components/Navbar';
import { useAuth } from 'hooks/useAuth';
import { Link } from 'react-router-dom';


export default function Account() {
  const auth = useAuth();

  return <>
    <Navbar />
    <Container>
      <Card>
        {auth.user && auth.user.username && `Hi, ${auth.user.username}!`}

        {auth.isLoggedIn() ? 'You are logged in!' : 'You are not logged in.'}
      </Card>

      <Link to="/logout">
        <Button>Log out</Button>
      </Link>
    </Container>
  </>;
}
