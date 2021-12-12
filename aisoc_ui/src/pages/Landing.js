import Card from 'components/Card';
import Navbar from 'components/Navbar';
import { Link } from 'react-router-dom';
import './Landing.scss';



export default function Landing({ competitions }) {
  return <>
    <Navbar />
    <div className='container'>
      <Card darker>
        <h1 style={{
          'textUnderlineOffset': '0.65rem',
          'textDecoration': 'underline',
          'fontSize': '3rem',
          'marginTop': 0,
          'marginBottom': '0.65rem',
        }}>Doxa</h1>
      </Card>
      <div className='open-competitions-label'>
        OPEN COMPETITIONS
      </div>
      {Object.keys(competitions).map(competition => <Card key={competition} className='competition-summary-card'>
        <h2>
          <Link
            to={`/c/${competition}/`}
            className='competition-summary-link'
          >{competitions[competition].name}</Link>
        </h2>
      </Card>)}
    </div>
  </>;
}
