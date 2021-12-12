import Card from 'components/Card';
import Navbar from 'components/Navbar';
import { useAuth } from 'hooks/useAuth';


export default function Account() {
  const auth = useAuth();

  return <>
    <Navbar />
    <div className='container'>
      <Card>
        {auth.isLoggedIn() ? 'You are logged in!' : 'You are not logged in.'}
      </Card>
    </div>
  </>;
}
