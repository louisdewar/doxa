import { useRef } from 'react';
import { Link } from 'react-router-dom';
import berkeleyAvifLogo from '../assets/logos/berkeley.avif';
import berkeleyLogo from '../assets/logos/berkeley.png';
import bristolAvifLogo from '../assets/logos/bristol.avif';
import bristolLogo from '../assets/logos/bristol.png';
import caltechAvifLogo from '../assets/logos/caltech.avif';
import caltechLogo from '../assets/logos/caltech.png';
import cambridgeAvifLogo from '../assets/logos/cambridge.avif';
import cambridgeLogo from '../assets/logos/cambridge.png';
import carnegieAvifLogo from '../assets/logos/carnegie-mellon.avif';
import carnegieLogo from '../assets/logos/carnegie-mellon.png';
import columbiaAvifLogo from '../assets/logos/columbia.avif';
import columbiaLogo from '../assets/logos/columbia.png';
import cornellAvifLogo from '../assets/logos/cornell.avif';
import cornellLogo from '../assets/logos/cornell.png';
import edinburghAvifLogo from '../assets/logos/edinburgh.avif';
import edinburghLogo from '../assets/logos/edinburgh.png';
import georgiaTechAvifLogo from '../assets/logos/georgia-tech.avif';
import georgiaTechLogo from '../assets/logos/georgia-tech.png';
import glasgowAvifLogo from '../assets/logos/glasgow.avif';
import glasgowLogo from '../assets/logos/glasgow.png';
import harvardAvifLogo from '../assets/logos/harvard.avif';
import harvardLogo from '../assets/logos/harvard.png';
import illinoisAvifLogo from '../assets/logos/illinois.avif';
import illinoisLogo from '../assets/logos/illinois.png';
import imperialAvifLogo from '../assets/logos/imperial.avif';
import imperialLogo from '../assets/logos/imperial.png';
import manchesterAvifLogo from '../assets/logos/manchester.avif';
import manchesterLogo from '../assets/logos/manchester.png';
import michiganAvifLogo from '../assets/logos/michigan.avif';
import michiganLogo from '../assets/logos/michigan.png';
import mitAvifLogo from '../assets/logos/mit.avif';
import mitLogo from '../assets/logos/mit.png';
import oxfordAvifLogo from '../assets/logos/oxford.avif';
import oxfordLogo from '../assets/logos/oxford.png';
import princetonAvifLogo from '../assets/logos/princeton.avif';
import princetonLogo from '../assets/logos/princeton.png';
import stAndrewsAvifLogo from '../assets/logos/st-andrews.avif';
import stAndrewsLogo from '../assets/logos/st-andrews.png';
import stanfordAvifLogo from '../assets/logos/stanford.avif';
import stanfordLogo from '../assets/logos/stanford.png';
import torontoAvifLogo from '../assets/logos/toronto.avif';
import torontoLogo from '../assets/logos/toronto.png';
import uclAvifLogo from '../assets/logos/ucl.avif';
import uclLogo from '../assets/logos/ucl.png';
import uclaAvifLogo from '../assets/logos/ucla.avif';
import uclaLogo from '../assets/logos/ucla.png';
import warwickAvifLogo from '../assets/logos/warwick.avif';
import warwickLogo from '../assets/logos/warwick.png';
import waterlooAvifLogo from '../assets/logos/waterloo.avif';
import waterlooLogo from '../assets/logos/waterloo.png';
import Footer from '../components/Footer';
import SplashHeader from '../components/SplashHeader';
import SplashNavbar from '../components/SplashNavbar';
import './Splash.scss';


export default function Splash({ baseUrl }) {
  const scrollRef = useRef(null);

  const logoMargin = getComputedStyle(document.body).getPropertyValue('--ch-uni-logo-margin');

  const logo = (src, srcAvif, university, p = '0em', m = '0em', etc = {}) => <picture>
    <source type="image/avif" srcSet={srcAvif} />

    <img
      src={src}
      style={{
        paddingTop: p ?? '0', paddingBottom: p ?? '0',
        marginLeft: `calc(${logoMargin} - ${m})`,
        marginRight: `calc(${logoMargin} - ${m})`,
        ...etc
      }}
      alt={`${university} logo`}
    />
  </picture>;

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
              The winning entry has the chance to be deployed by the UK National Grid Electricity System Operator to minimise the use of standby gas turbines, potentially resulting in carbon emission savings of up to 100 kilotonnes a year.
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
          <div className='ch-splash-universities'>
            <div className='ch-splash-universities-logos'>
              {logo(uclLogo, uclAvifLogo, 'UCL', undefined, '0.25em')}
              {logo(stanfordLogo, stanfordAvifLogo, 'Stanford', '0.6em')}
              {logo(berkeleyLogo, berkeleyAvifLogo, 'UC Berkeley', '0.65em', '0.2em')}
              {logo(oxfordLogo, oxfordAvifLogo, 'Oxford', '0.75em', '0.25em')}
              {logo(cambridgeLogo, cambridgeAvifLogo, 'Cambridge', '0.25em')}
              {logo(mitLogo, mitAvifLogo, 'MIT', '1em')}
              {logo(torontoLogo, torontoAvifLogo, 'Toronto', '0.2em', '0.6em')}
              {logo(harvardLogo, harvardAvifLogo, 'Harvard', '0.225em', undefined, { marginRight: '-0.4em' })}
              {logo(princetonLogo, princetonAvifLogo, 'Princeton', '0.9em')}
              {logo(imperialLogo, imperialAvifLogo, 'Imperial', '1em', '-0.1em')}
              {logo(cornellLogo, cornellAvifLogo, 'Cornell', '0.5em', '0.5em')}
              {logo(caltechLogo, caltechAvifLogo, 'Caltech', '0.2em', '0.8em')}
              {logo(carnegieLogo, carnegieAvifLogo, 'Carnegie Mellon', '0.65em')}
              {logo(uclaLogo, uclaAvifLogo, 'UCLA', '1em')}
              {logo(columbiaLogo, columbiaAvifLogo, 'Columbia', '0.4em')}
              {logo(stAndrewsLogo, stAndrewsAvifLogo, 'St Andrews', '0.25em', '0.2em')}
              {logo(georgiaTechLogo, georgiaTechAvifLogo, 'Georgia Tech', '0.9em')}
              {logo(edinburghLogo, edinburghAvifLogo, 'Edinburgh', '0.8em')}
              {logo(manchesterLogo, manchesterAvifLogo, 'Manchester', '0.4em', '0.4em')}
              {logo(waterlooLogo, waterlooAvifLogo, 'Waterloo', undefined, '0.3em')}
              {logo(michiganLogo, michiganAvifLogo, 'Michigan', '1rem')}
              {logo(warwickLogo, warwickAvifLogo, 'Warwick', undefined, '2.25em')}
              {logo(bristolLogo, bristolAvifLogo, 'Bristol', '0.4em')}
              {logo(illinoisLogo, illinoisAvifLogo, 'Illinois', '0.15em', '0.75em')}
              {logo(glasgowLogo, glasgowAvifLogo, 'Glasgow', undefined, '1.75em')}
            </div>

            <p>
              Climate Hack.AI is open to all students at any of the co-hosting universities, no matter their level of machine learning experience. This is an incredible opportunity to learn something new, as societies will be releasing educational resources over the course of the competition.
            </p>
            <p>
              Beyond this datathon, Climate Hack.AI aims to foster a community of students interested in artificial intelligence from around the world.
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
              Climate Hack.AI is a two-month-long datathon based on climate-related data provided by our partner, Open Climate Fix.
            </p>
            <p>
              Competitors are challenged to train the best machine learning models using 108GB of satellite imagery. Submissions are evaluated against an unseen test dataset on our own custom competition platform, <Link to={`${baseUrl}compete`}>DOXA</Link>.
            </p>
            <p>
              By improving on the state of the art in nowcasting satellite imagery, the winning model could be deployed by the National Grid electricity system operator in the UK to produce significantly more accurate solar photovoltaic output forecasts.
            </p>
            <p>
              This would allow them to minimise the use of standby gas turbines, potentially leading to a substantial reduction in carbon emissions of up to 100 kilotonnes a year.
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
                  Participants gain access to the dataset to start training models to be submitted individually.
                </p>
                <p>
                  The top three competitors from each university will then be invited to the finals, along with a society representative.
                </p>
              </div>
              <div>
                <p>
                  Teams of three finalists representing their respective universities compete to improve their models.
                </p>
                <p>
                  Their final submissions will be used to determine the winning team by a panel of judges at the in-person final events in London and New York.
                </p>
              </div>
            </div>
            <h3>Participation Requirements</h3>
            <p>
              Participants must be attending one of the co-hosting universities as an undergraduate, masters or PhD student at the time of the competition.
            </p>
            <p>
              Submissions in the first round are individual; however, we encourage collaboration between participants. It may be a competition, but everyone is working as a team to beat climate change.
            </p>
            <p>
              Detailed competition rules and submission instructions may be found on our competition platform, <Link to={`${baseUrl}compete#4`}>DOXA</Link>.
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
