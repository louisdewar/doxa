import { faCaretDown } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useHistory } from 'react-router-dom/cjs/react-router-dom.min';
import ennovate from '../assets/ennovate.png';
import newcross from '../assets/newcross-white-orange.png';
import './SplashHeader.scss';



export default function SplashHeader({ baseUrl, scroll }) {
  const history = useHistory();

  return <header className='ch-splash-header'>
    <div className='ch-splash-header-title'>
      <div className='ch-splash-header-title-content'>
        <h1 className='ch-intro-title'>Climate</h1>
        <h2>Hack.<span>AI</span></h2>

        <button className='ch-button ch-compete-button' onClick={() => {
          history.push(`${baseUrl}compete`);
        }}>Compete now</button>
        <button className='ch-button ch-linktree-button' onClick={() => {
          window.location.href = 'https://linktr.ee/climatehack.ai';
        }}>LinkTree</button>
      </div>
    </div>

    <div className='ch-scroll-to-about'>
      <div>
        <a href="https://entaingroup.com/">
          <span className='sr'>Scroll down</span>
          <img src={ennovate} style={{ width: '150px' }} alt="Ennovate logo" />
        </a>
      </div>
      <div>
        <a href="#" onClick={e => {
          e.preventDefault();
          scroll();
        }}><FontAwesomeIcon icon={faCaretDown} fixedWidth /></a>
      </div>
      <div>
        <a href="https://www.newcrosshealthcare.com/"><img src={newcross} style={{ height: '2rem' }} alt="Newcross Healthcare logo" /></a>
      </div>
    </div>
  </header>;
}
