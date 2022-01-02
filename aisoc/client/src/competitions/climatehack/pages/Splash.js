import { faCaretDown } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { useRef } from 'react';
import { Link, useHistory } from 'react-router-dom';
import berkeleyLogo from '../assets/logos/berkeley.png';
import caltechLogo from '../assets/logos/caltech.png';
import cambridgeLogo from '../assets/logos/cambridge.png';
import carnegieLogo from '../assets/logos/carnegie-mellon.png';
import columbiaLogo from '../assets/logos/columbia.png';
import cornellLogo from '../assets/logos/cornell.png';
import edinburghLogo from '../assets/logos/edinburgh.png';
import georgiaTechLogo from '../assets/logos/georgia-tech.png';
import harvardLogo from '../assets/logos/harvard.png';
import illinoisLogo from '../assets/logos/illinois.png';
import imperialLogo from '../assets/logos/imperial.png';
import michiganLogo from '../assets/logos/michigan.png';
import mitLogo from '../assets/logos/mit.png';
import oxfordLogo from '../assets/logos/oxford.png';
import princetonLogo from '../assets/logos/princeton.png';
import torontoLogo from '../assets/logos/toronto.png';
import uclLogo from '../assets/logos/ucl.png';
import uclaLogo from '../assets/logos/ucla.png';
import waterlooLogo from '../assets/logos/waterloo.png';
import SplashNavbar from '../components/SplashNavbar';
import './Splash.scss';


export default function Splash({ baseUrl }) {
  const history = useHistory();
  const aboutRef = useRef(null);

  return <div className='ch-wrapper'>
    <SplashNavbar baseUrl={baseUrl} />

    <header className='ch-splash-header'>
      <div className='ch-splash-header-title'>
        <h1 className='ch-intro-title'><span>Climate</span></h1>
        <h2><span>Hack</span></h2>

        <button className='ch-compete-button' onClick={() => {
          history.push(`${baseUrl}compete`);
        }}>Compete on DOXA</button>
      </div>

      <div className='ch-scroll-to-about'>
        <a href="#" onClick={e => {
          e.preventDefault();

          window.scrollTo({
            top: aboutRef.current.getBoundingClientRect().top + window.scrollY - 75,
            behavior: 'smooth'
          });

          // aboutRef.current.scrollIntoView({ behavior: 'smooth', block: 'start' });
        }}><FontAwesomeIcon icon={faCaretDown} fixedWidth /></a>
      </div>
    </header>

    <section className='ch-splash-section ch-splash-about' ref={aboutRef}>
      <div className='ch-splash-section-content'>
        <div>
          <p>
            Climate Hack is a collaborative initiative between the student communities of <strong>leading CS and AI universities</strong> across the <strong>US, UK and Canada</strong>, applying the cutting edge of machine learning to join the <strong>fight against climate change</strong>.
          </p>
          <p>
            Participants have 3 months to develop the best <strong>solar photovoltaic forecasting algorithm</strong>, with the chance to be deployed to the UK National grid to <strong>reduce national carbon emissions</strong>.
          </p>
        </div>
      </div>
    </section>

    <div className='ch-splash-impact-stripe'>
      <p>
        Make an impact. Meet people. Win big.
      </p>
    </div>

    <section className='ch-splash-section ch-splash-universities'>
      <div className='ch-splash-section-content'>
        <div className='ch-splash-section-heading'>
          <h2>
            25 Universities
          </h2>
          <p>
            Jointly organised by the student communities of 25 top-ranking CS universities from the US, UK and Canada.
          </p>
        </div>
        <div className='ch-splash-universities-logos'>
          <img src={berkeleyLogo} style={{ padding: '0.5rem 0', margin: '0.5rem -0.2rem' }} />
          <img src={caltechLogo} style={{ margin: '0.5rem -0.8rem' }} />
          <img src={cambridgeLogo} />
          <img src={carnegieLogo} />
          <img src={columbiaLogo} />
          <img src={cornellLogo} style={{ margin: '0.5rem -1rem' }} />
          <img src={edinburghLogo} style={{ padding: '0.75rem 0' }} />
          <img src={georgiaTechLogo} style={{ padding: '0.75rem 0' }} />
          <img src={harvardLogo} style={{ marginRight: '-1rem' }} />
          <img src={illinoisLogo} style={{ margin: '0.5rem -0.75rem' }} />
          <img src={imperialLogo} style={{ padding: '1rem 0' }} />
          <img src={michiganLogo} style={{ padding: '0.75rem 0' }} />
          <img src={mitLogo} style={{ padding: '0.75rem 0' }} />
          <img src={oxfordLogo} style={{ padding: '0.75rem 0' }} />
          <img src={princetonLogo} style={{ padding: '1rem 0' }} />
          <img src={torontoLogo} style={{ margin: '0.5rem -0.5rem' }} />
          <img src={uclLogo} />
          <img src={uclaLogo} style={{ padding: '1rem 0' }} />
          <img src={waterlooLogo} />
        </div>
      </div>
    </section>

    <section className='ch-splash-section ch-splash-format'>
      <div className='ch-splash-section-content'>
        <div className='ch-splash-section-heading'>
          <h2>
            Competition Format
          </h2>
        </div>
        <div className='ch-splash-format-description'>
          <div className='ch-splash-format-columns'>
            <div>
              <h3>Virtual phase</h3>
              <h4>28th January</h4>
            </div>

            <div>
              <h3>In-person finals</h3>
              <h4>24-26th March</h4>
            </div>
          </div>

          <svg width="530" height="36" viewBox="0 0 530 36" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M24 14H517V23H24V14Z" fill="url(#paint0_linear_266_3)" />
            <circle cx="17.5" cy="18.5" r="17.5" fill="#DC1C5F" />
            <ellipse cx="512.5" cy="18" rx="17.5" ry="18" fill="#0066FF" />
            <defs>
              <linearGradient id="paint0_linear_266_3" x1="24" y1="23.0002" x2="517" y2="22.9998" gradientUnits="userSpaceOnUse">
                <stop stopColor="#DC1C5F" />
                <stop offset="0.567708" stopColor="#7444AE" />
                <stop offset="0.9999" stopColor="#2563EB" />
                <stop offset="1" stopColor="#0066FF" />
              </linearGradient>
            </defs>
          </svg>

          <div className='ch-splash-format-columns'>
            <div>
              <p>
                Participants will produce models using the provided dataset to be submitted either individually or in teams of up to three. The top 3 competitors from each university will then be invited to the finals.
              </p>
            </div>

            <div>
              <p>
                Finalists will get access to an expanded training dataset and work within a team representing their respective universities to determine an overall winning model.
              </p>
            </div>
          </div>

          <h3>Participation Requirements</h3>
          <p>
            Participants must be attending one of our partner universities as an undergraduate, masters or PhD student.
          </p>

          <p>
            Detailed competition rules and submission instructions may be found on our competition platform <Link to={`${baseUrl}compete`}>DOXA</Link>.
          </p>
        </div>
      </div>
    </section>

    <section className='ch-splash-section ch-splash-prizes'>
      <div className='ch-splash-section-content'>
        <div className='ch-splash-section-heading'>
          <h2>
            Prizes
          </h2>
          <h3>First-place team</h3>
          <span>£30k</span>
          <h3>Second-place team</h3>
          <span>£9k</span>
          <h3>Third-place team</h3>
          <span>£6.75k</span>
        </div>
        <div className='ch-splash-prizes-descriptions'>
          <p>
            The top 3 teams will share a prize pool of
          </p>
          <span>
            £50k ($68k)
          </span>
          <br />
          <br />
          <br />
          <p>
            In addition, the societies the top 3 teams represent will receive the following cash prizes:
          </p>
          <div className='ch-splash-prizes-columns'>
            <div>
              <h3>First-place team</h3>
              <span>£2.5k</span>
            </div>
            <div>
              <h3>Second-place team</h3>
              <span>£1k</span>
            </div>
            <div>
              <h3>Third-place team</h3>
              <span>£750</span>
            </div>
          </div>

        </div>
      </div>
    </section>

    <section className='ch-splash-section ch-splash-tangible-impact'>
      <div className='ch-splash-section-content'>
        <div className='ch-splash-section-heading'>
          <h2>
            Make a Tangible Impact
          </h2>
        </div>
        <div className='ch-splash-tangible-impact-description'>
          <p>
            Climate Hack is a 3-month long datathon, for which our partner organisation, OpenClimateFix, provides a dataset of climate-related data.
          </p>
          <p>
            Competitors are tasked with developing machine learning models trained on this data, which are then automatically evaluated against a test set for their expected capability to reduce carbon emissions.
          </p>
          <p>
            The winning model may be deployed to the UK National Grid.
          </p>
          <p>
            Learn more about the <Link to={`${baseUrl}challenge`}>challenge</Link>.
          </p>
        </div>
      </div>
    </section>




    <br />    <br />
    <br />    <br />
    <br />    <br />
    <br />    <br />
    <br />    <br />
    <br />    <br />
    <br />    <br />
    <br />    <br />
    <br />    <br />
    <br />    <br />
    <br />    <br />
    <br />    <br />
    <br />    <br />
    <br />    <br />
    <br />    <br />
    <br />    <br />
    <br />    <br />
    <br />    <br />
    <br />    <br />
    <br />    <br />



  </div>;
}
