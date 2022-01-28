import Button from 'components/Button';
import Card from 'components/Card';
import { Link } from 'react-router-dom';

export default function DelegatedSuccess() {
  return <>
    <Card>
      <h2>Successfully authorised the delegated login</h2>
      <p>
        You may now close this window.
      </p>
      <Link to="/">
        <Button success>Return to the hompage</Button>
      </Link>
    </Card>
  </>;
}
