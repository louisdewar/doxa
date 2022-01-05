import Card from 'components/Card';
import Container from 'components/Container';
import SplashNavbar from '../components/SplashNavbar';


export default function ComingSoon({ baseUrl }) {
  return <>
    <div className='ch-wrapper'>
      <SplashNavbar baseUrl={baseUrl} />

      <header className='ch-challenge-header' style={{ textAlign: 'center' }}>
        <h1>
          Coming soon
        </h1>
      </header>
    </div>

    <Container padTop={false}>

      <div style={{ marginTop: '2rem', fontSize: '1.2rem' }}>
        <span></span>
        <span></span>

        <Card>
          <p>
            The official Climate Hack competition page on DOXA will launch on Friday 28th January.
          </p>
          <p>
            We look forward to seeing you then!
          </p>
          <p>
            &mdash; <strong>The DOXA team</strong>
          </p>
        </Card>
      </div>
    </Container>






  </>;
}
