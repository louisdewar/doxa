import { useAuth } from 'hooks/useAuth';
import './Navbar.scss';

export default function Navbar({ competition, competitionName }) {
  const auth = useAuth();

  return <nav className='nav'>
    <a href="/">DOXA</a>
    {competitionName && <a href={`/c/${competition}/`} className='navbar-active'>{competitionName}</a>}
    {auth.isLoggedIn()
      ? <a href='/account' className='account'>ACCOUNT</a>
      : <a href='/login' className='account'>LOGIN</a>}
  </nav>;
}
