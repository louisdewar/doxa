import { useAuth } from 'hooks/useAuth';
import { Link } from 'react-router-dom';
import './Navbar.scss';

export default function Navbar({ competition, competitionName, competitionLink }) {
  const auth = useAuth();

  return <nav className='nav'>
    <Link to="/">DOXA</Link>
    {competitionName && <Link to={competitionLink ?? `/c/${competition}/`} className='navbar-active'>{competitionName}</Link>}
    {auth.isLoggedIn()
      ? <Link to='/account' className='account'>ACCOUNT</Link>
      : <Link to='/login' className='account'>LOGIN</Link>}
  </nav>;
}
