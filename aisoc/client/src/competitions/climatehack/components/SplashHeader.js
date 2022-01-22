import { faCaretDown } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useHistory } from 'react-router-dom/cjs/react-router-dom.min';
import './SplashHeader.scss';


export default function SplashHeader({ baseUrl, scroll }) {
  const history = useHistory();

  return <header className='ch-splash-header'>
    <div className='ch-splash-header-title'>
      <div className='ch-splash-header-title-content'>
        <h1 className='ch-intro-title'>Climate</h1>
        <h2>Hack.<span>AI</span></h2>

        <button className='ch-compete-button' onClick={() => {
          history.push(`${baseUrl}comingsoon`);
        }}>Compete on DOXA</button>
      </div>
    </div>

    <div className='ch-scroll-to-about'>
      <a href="#" onClick={e => {
        e.preventDefault();
        scroll();
      }}><FontAwesomeIcon icon={faCaretDown} fixedWidth /></a>
    </div>
  </header>;
}
