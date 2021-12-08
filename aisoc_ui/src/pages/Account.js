import { useAuth } from 'hooks/useAuth';


export default function Account() {
  const auth = useAuth();

  return <>
    {auth.isLoggedIn() ? 'You are logged in!' : 'You are not logged in.'}
  </>;
}
