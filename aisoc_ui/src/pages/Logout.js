import { useAuth } from 'hooks/useAuth';
import { useHistory } from 'react-router-dom';


export default function Logout() {
  const auth = useAuth();
  const history = useHistory();

  if (auth.isLoggedIn()) {
    auth.logout();
  }

  history.push('/');

  return <></>;
}
