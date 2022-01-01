import { faCaretDown } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import Container from 'components/Container';
import { useRef } from 'react';
import { useHistory } from 'react-router-dom';
import SplashNavbar from '../components/SplashNavbar';
import './Splash.scss';

export default function Splash({ baseUrl }) {
  const history = useHistory();

  return <div className='ch-wrapper'>
    <SplashNavbar baseUrl={baseUrl} />

    <header className='ch-splash-header'>
      <div className='ch-splash-header-title'>
        <h1 className='ch-intro-title'><span>Climate</span></h1>
        <h2><span>Hack</span></h2>

        <button className='ch-compete-button' onClick={() => {
          history.push(`${baseUrl}compete`);
        }}>Compete on Doxa</button>
      </div>

      <div className='ch-scroll-to-about'>
        <a href="#" onClick={e => {
          e.preventDefault();
        }}><FontAwesomeIcon icon={faCaretDown} fixedWidth /></a>
      </div>
    </header>

  </div>;
}
