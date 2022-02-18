import Card from 'components/Card';
import Container from 'components/Container';
import Footer from 'components/Footer';
import Navbar from 'components/Navbar';


export default function About() {
  return <div className='main-wrapper'>
    <Navbar />
    <Container>
      <span></span><span></span><span></span><span></span> {/* it's that sneaky colour hack again! */}
      <Card>
        <h1>About DOXA</h1>
        <p>
          <a href="https://github.com/louisdewar/doxa">DOXA</a> is a customisable, open-source platform for running fully automated AI competitions developed by <a href="https://louis.dewardt.uk/">Louis de Wardt</a> and <a href="https://jezz.me/">Jeremy Lo Ying Ping</a>, who make up the development team at the <a href="https://uclaisociety.co.uk/">UCL Artificial Intelligence Society</a>.
        </p>
        {/* TODO: how DOXA works */}
      </Card>
    </Container>
    <Footer />
  </div>;
}
