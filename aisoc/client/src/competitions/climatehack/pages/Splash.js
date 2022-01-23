import { useRef } from 'react';
import { Link } from 'react-router-dom';
import berkeleyLogo from '../assets/logos/berkeley.png';
import bristolLogo from '../assets/logos/bristol.png';
import caltechLogo from '../assets/logos/caltech.png';
import cambridgeLogo from '../assets/logos/cambridge.png';
import carnegieLogo from '../assets/logos/carnegie-mellon.png';
import columbiaLogo from '../assets/logos/columbia.png';
import cornellLogo from '../assets/logos/cornell.png';
import edinburghLogo from '../assets/logos/edinburgh.png';
import georgiaTechLogo from '../assets/logos/georgia-tech.png';
import glasgowLogo from '../assets/logos/glasgow.png';
import harvardLogo from '../assets/logos/harvard.png';
import illinoisLogo from '../assets/logos/illinois.png';
import imperialLogo from '../assets/logos/imperial.png';
import manchesterLogo from '../assets/logos/manchester.png';
import michiganLogo from '../assets/logos/michigan.png';
import mitLogo from '../assets/logos/mit.png';
import oxfordLogo from '../assets/logos/oxford.png';
import princetonLogo from '../assets/logos/princeton.png';
import stAndrewsLogo from '../assets/logos/st-andrews.png';
import stanfordLogo from '../assets/logos/stanford.png';
import torontoLogo from '../assets/logos/toronto.png';
import uclLogo from '../assets/logos/ucl.png';
import uclaLogo from '../assets/logos/ucla.png';
import warwickLogo from '../assets/logos/warwick.png';
import waterlooLogo from '../assets/logos/waterloo.png';
import Footer from '../components/Footer';
import SplashHeader from '../components/SplashHeader';
import SplashNavbar from '../components/SplashNavbar';
import './Splash.scss';


export default function Splash({ baseUrl }) {
  const scrollRef = useRef(null);

  const logoMargin = '0.75rem';
  const logo = (src, p = '0rem', m = '0rem', etc = {}) => <img
    src={src}
    style={{
      paddingTop: p ?? '0', paddingBottom: p ?? '0',
      marginLeft: `calc(${logoMargin} - ${m})`,
      marginRight: `calc(${logoMargin} - ${m})`,
      ...etc
    }}
  />;

  return <div className='ch-wrapper'>
    <SplashNavbar baseUrl={baseUrl} />

    <SplashHeader baseUrl={baseUrl} scroll={() => {
      window.scrollTo({
        top: scrollRef.current.getBoundingClientRect().top + window.scrollY - 95,
        behavior: 'smooth'
      });
    }} />

    <div className="ch-panel-container">
      <section className='ch-section ch-splash-about' ref={scrollRef}>
        <div className='ch-section-content ch-section-content'>
          <div className='ch-section-heading ch-section-heading'>
            <h2>About Climate Hack.AI</h2>
          </div>
          <div>
            <p>
              Climate Hack.AI is a collaborative initiative between the student communities of 25 universities leading in CS and AI from across the United States, the United Kingdom and Canada to take a lead in the fight against climate change.
            </p>
            <p>
              Participants have two months to apply cutting-edge machine learning techniques in order to develop the best satellite imagery prediction algorithm for use in solar photovolatic output forecasting.
            </p>
            <p>
              The winning entry has the chance to be deployed in the UK National Grid to minimise the use of idle gas turbines, potentially resulting in significant savings in national carbon emissions.
            </p>
          </div>
        </div>
      </section >

      <section className='ch-section ch-splash-universities'>
        <div className='ch-section-content ch-section-content'>
          <div className='ch-section-heading ch-section-heading'>
            <h2>
              Connect with Participants from 25 Universities
            </h2>
          </div>
          <div className='ch-splash-universities-logos'>
            {logo(uclLogo, undefined, '0.25rem')}
            {logo(stanfordLogo, '0.6rem')}
            {logo(berkeleyLogo, '0.65rem', '0.2rem')}
            {logo(oxfordLogo, '0.75rem', '0.25rem')}
            {logo(cambridgeLogo, '0.25rem')}
            {logo(mitLogo, '1.3rem')}
            {logo(torontoLogo, '0.2rem', '0.6rem')}
            {logo(harvardLogo, '0.225rem', undefined, { marginRight: '-0.4rem' })}
            {logo(princetonLogo, '0.9rem')}
            {logo(imperialLogo, '1rem', '-0.1rem')}
            {logo(cornellLogo, '0.5rem', '0.5rem')}
            {logo(caltechLogo, '0.2rem', '0.8rem')}
            {logo(carnegieLogo, '0.65rem')}
            {logo(uclaLogo, '1rem')}
            {logo(columbiaLogo, '0.4rem')}
            {logo(stAndrewsLogo, '0.25rem', '0.2rem')}
            {logo(georgiaTechLogo, '0.9rem')}
            {logo(edinburghLogo, '0.8rem')}
            {logo(manchesterLogo, '0.4rem', '0.4rem')}
            {logo(waterlooLogo, undefined, '0.3rem')}
            {logo(michiganLogo, '1rem')}
            {logo(warwickLogo, undefined, '2.25rem')}
            {logo(bristolLogo, '0.4rem')}
            {logo(illinoisLogo, '0.15rem', '0.75rem')}
            {logo(glasgowLogo, undefined, '1.75rem')}

            <p>
              Beyond this competition, Climate Hack.AI aims to foster a community of students interested in artificial intelligence from around the world.
            </p>
            <p>
              Throughout the competition, there will be plenty of internal and cross-university events for students to meet each other and showcase their knowledge of AI, including in-person and virtual social events, mini-competitions and cross-university tutorials.
            </p>
          </div>
        </div>
      </section>

      <section className='ch-section ch-splash-tangible-impact'>
        <div className='ch-section-content ch-section-content'>
          <div className='ch-section-heading ch-section-heading'>
            <h2>
              Make a Tangible Impact
            </h2>
          </div>
          <div className='ch-splash-tangible-impact-description'>
            <p>
              Climate Hack.AI is a two-month-long datathon based on climate-related data provided by our partner, OpenClimateFix.
            </p>
            <p>
              Competitors are challenged to develop the best machine learning models trained on this data. Submissions are evaluated against an unseen test dataset on our own custom competition platform, <Link to={`${baseUrl}comingsoon`}>DOXA</Link>.
            </p>
            <p>
              By improving on the state of the art in nowcasting satellite imagery, the winning model could be deployed by the British electricity system operator (National Grid) to produce significantly more accurate solar photovoltaic output forecasts.
            </p>
            <p>
              This would allow them to minimise the use of idle gas turbines, potentially leading to a substantial reduction in carbon emissions.
            </p>
            <p>
              Learn more about the <Link to={`${baseUrl}challenge`}>challenge</Link>.
            </p>
          </div>
        </div>
      </section>

      <section className='ch-section ch-splash-prizes'>
        <div className='ch-section-content ch-section-content'>
          <div className='ch-section-heading ch-section-heading'>
            <h2>
              A £50k Prize Pool
            </h2>
          </div>
          <div className='ch-splash-prizes-descriptions'>
            <p>
              The top 3 student teams will receive the following cash prizes:
            </p>
            <div className='ch-splash-prizes-columns'>
              <div>
                <h3>1<sup>st</sup> place team</h3>
                <span>£30k</span>
              </div>
              <div>
                <h3>2<sup>nd</sup> place team</h3>
                <span>£9k</span>
              </div>
              <div>
                <h3>3<sup>rd</sup> place team</h3>
                <span>£6.75k</span>
              </div>
            </div>
            <br />
            <p>
              Their respective societies will receive the following cash prizes:
            </p>
            <div className='ch-splash-prizes-columns'>
              <div>
                <h3>1<sup>st</sup> place society</h3>
                <span>£2.5k</span>
              </div>
              <div>
                <h3>2<sup>nd</sup> place society</h3>
                <span>£1k</span>
              </div>
              <div>
                <h3>3<sup>rd</sup> place society</h3>
                <span>£750</span>
              </div>
            </div>
          </div>
        </div>
      </section>

      <section className='ch-section ch-splash-format'>
        <div className='ch-section-content ch-section-content'>
          <div className='ch-section-heading ch-section-heading'>
            <h2>
              Competition Format
            </h2>
          </div>
          <div className='ch-splash-format-description'>
            <div className='ch-splash-format-columns'>
              <div>
                <h3>Launch</h3>
                <h4>28th January</h4>
              </div>
              <div>
                <h3>In-person finals</h3>
                <h4>24-26th March</h4>
              </div>
            </div>

            <svg width="530" height="36" viewBox="0 0 530 36" fill="none" xmlns="http://www.w3.org/2000/svg">
              <ellipse cx="512.5" cy="18" rx="17.5" ry="18" fill="#0066FF" />
              <path d="M24 14H517V23H24V14Z" fill="url(#paint0_linear_0_1)" />
              <circle cx="17.5" cy="18.5" r="17.5" fill="#83C554" />
              <defs>
                <linearGradient id="paint0_linear_0_1" x1="24" y1="23.0002" x2="517" y2="22.9998" gradientUnits="userSpaceOnUse">
                  <stop stopColor="#83C554" />
                  <stop offset="1" stopColor="#2563EB" />
                </linearGradient>
              </defs>
            </svg>

            <div className='ch-splash-format-columns'>
              <div>
                <p>
                  Participants train models on the provided dataset to be submitted either individually or in teams of up to three.
                </p>
                <p>
                  The top three competitors from each university will then be invited to the finals, along with a society representative.
                </p>
              </div>
              <div>
                <p>
                  Finalists gain access to an expanded training dataset and work in teams of three representing their respective universities.
                </p>
                <p>
                  The final models they submit will be used to determine the winning team.
                </p>
              </div>
            </div>
            <h3>Participation Requirements</h3>
            <p>
              Participants must be attending one of the co-hosting universities as an undergraduate, masters or PhD student at the time of the competition.
            </p>
            <p>
              Detailed competition rules and submission instructions may be found on our competition platform, <Link to={`${baseUrl}comingsoon`}>DOXA</Link>.
            </p>
          </div>
        </div>
      </section>

      <section className='ch-section ch-splash-joint-final'>
        <div className='ch-section-content ch-section-content'>
          <div className='ch-section-heading ch-section-heading'>
            <h2>
              Joint Final in New York and London
            </h2>
          </div>
          <div className='ch-splash-joint-final-description'>
            <p>
              The competition will conclude with an <strong>in-person</strong> final weekend for the top 3 competitors from each university.
            </p>
            <p>
              There will be two simultaneous final events hosted in New York and London for finalists in North America and London, respectively. All transport, accommodation and carbon offsetting expenses will be paid for. The winning team will be selected and announced on the last day of the competition.
            </p>
          </div>
        </div>
      </section>
    </div>

    <Footer />
  </div >;
}
