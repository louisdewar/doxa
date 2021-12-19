import { useAuth } from 'hooks/useAuth';
import { useEffect } from 'react';
import { Redirect } from 'react-router-dom';


export default function Logout() {
  const auth = useAuth();

  if (auth.isLoggedIn()) {
    useEffect(auth.logout, []);
  }

  return <Redirect to='/' />;
}
