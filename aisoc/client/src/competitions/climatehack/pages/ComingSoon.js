import SplashNavbar from '../components/SplashNavbar';
import './ComingSoon.scss';


export default function ComingSoon({ baseUrl }) {
  return <>
    <div className='ch-wrapper'>
      <SplashNavbar baseUrl={baseUrl} />

      <header className='ch-challenge-header' style={{ textAlign: 'center' }}>
        <h1>
          Coming soon
        </h1>
      </header>

      <div className='ch-panel-container'>
        <div className='ch-coming-soon-card'>
          <p>
            The official Climate Hack.AI competition page on DOXA will launch on Friday 28th January.
          </p>
          <p>
            We look forward to seeing you then!
          </p>
          <p>
            &mdash; <strong>The DOXA team</strong>
          </p>
        </div>
      </div>
    </div>
  </>;
}
