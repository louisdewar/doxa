import { Link } from 'react-router-dom';
import './Footer.scss';

export default function Footer() {
  return <footer className='doxa-footer'>
    <div className='doxa-footer-about-us'>
      <Link to="/about">About DOXA</Link>
    </div>
    <div className='doxa-footer-copyright'>
      Copyright &copy; {new Date().getFullYear()}
    </div>
  </footer>;
}
