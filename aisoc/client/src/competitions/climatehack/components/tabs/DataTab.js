import satelliteImage from '../../assets/satellite-image.png';

export default function DataTab() {
  return <div className="ch-tab">
    <h2>Data</h2>
    <img src={satelliteImage} style={{ width: '100%', backgroundColor: '#f1f5f9', borderRadius: '3px', padding: '1rem', boxSizing: 'border-box' }} />
    <p>
      Hello!
    </p>
  </div>;
}
