import { Link } from 'react-router-dom';
import './NavBar.scss';

export default function Navbar({ competitionName, homepageUrl }) {
  return (
    <nav>
      <Link to={homepageUrl}><p className="comp-name">{competitionName}</p></Link>
      <ul className="nav-links">
        <li><Link to='/account'>Account</Link></li>
      </ul>
    </nav>
  );
}
