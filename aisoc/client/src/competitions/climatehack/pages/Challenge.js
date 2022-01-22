import ocfLogo from '../assets/ocf-logo.png';
import Footer from '../components/Footer';
import SplashNavbar from '../components/SplashNavbar';
import './Challenge.scss';

export default function Challenge({ baseUrl }) {
  return <div className='ch-wrapper'>
    <SplashNavbar baseUrl={baseUrl} />

    <header className='ch-challenge-header'>
      <h2>
        The
      </h2>
      <h1>
        Challenge
      </h1>
    </header>

    <div className='ch-panel-container'>
      <section className='ch-section ch-challenge-pv'>
        <div className='ch-section-content'>
          <div className='ch-section-heading'>
            <h2>
              Improving Solar Photovoltaic Power Production Forecasts
            </h2>
          </div>
          <div>
            <p>
              There is a great deal of uncertainty in the forecasting of Solar Photovoltaic (PV) power production in the UK. Accordingly, the National Grid needs to keep natural gas turbines idle in the event of a fall in PV production (for example, when a large cloud covers a solar farm).
            </p>
            <p>
              If the National Grid could more accurately forecast PV output, the usage of these gas turbines could be reduced due to the lessened uncertainty, consequently reducing the associated carbon emissions. Our task is to beat the current algorithm forecasting cloud coverage in the UK to better predict PV power production.
            </p>
          </div>
        </div>

      </section>

      <section className='ch-section ch-challenge-ocf'>
        <div className='ch-section-content'>
          <div className='ch-section-heading'>
            <img src={ocfLogo} />
          </div>
          <div>
            <p>
              Our partner and data provider, OpenClimateFix (OCF), is a private, non-profit lab seeking to use Machine Learning to reduce industrial carbon emissions. OCF is working with the UK National Grid Electricity System Operator to address this challenge, and will provide 1.5 years of satellite training data, in addition to publishing their current, state-of-the-art model&apos;s weights for us to build upon.
            </p>
          </div>
        </div>

      </section>

      <section className='ch-section ch-challenge-impact'>
        <div className='ch-section-content'>
          <div className='ch-section-heading'>
            <h2>
              Potential Impact
            </h2>
          </div>
          <div>
            <p>
              Improved PV forecasting for the National Grid has the potential to save up to 100 kilotonnes of carbon emissions per year in the UK (up to 1.3 megatonnes by 2030). OpenClimateFix also plans to deploy these solutions abroad, which should result in even more substantial carbon savings globally.
            </p>
          </div>
        </div>
      </section>
    </div>

    <Footer />

  </div>;
}
