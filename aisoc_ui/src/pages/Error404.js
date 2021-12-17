import Button from 'components/Button';
import Card from 'components/Card';
import Container from 'components/Container';
import Navbar from 'components/Navbar';
import { Link } from 'react-router-dom';


export default function Error404() {
  return <>
    <Navbar />
    <Container>
      <Card>
        <h1>Error 404</h1>
        <p>
          Unfortunately, the page for which you were looking could not be found.
        </p>
      </Card>

      <Link to="/">
        <Button success>Return to the home page</Button>
      </Link>
    </Container>
  </>;
}
