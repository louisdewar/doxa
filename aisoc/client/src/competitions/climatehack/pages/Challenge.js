import ocfLogo from '../assets/ocf-white.png';
import satelliteImage from '../assets/satellite-image.png';
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
      <section className='ch-section ch-challenge-ocf'>
        <div className='ch-section-content'>
          <div className='ch-section-heading'>
            <img src={ocfLogo} />
          </div>
          <div className='ch-section-description'>
            <p>
              We have partnered with Open Climate Fix: a non-profit product lab focused on reducing greenhouse gas emissions as rapidly as possible with machine learning.
            </p>

            <p>
              Every part of their organisation is designed to maximise climate impact from their open and collaborative approach to their rapid prototyping and attention on finding scalable & practical solutions. By using an open-source approach, they can draw upon a much larger pool of expertise than any individual company, thereby combining existing islands of knowledge and accelerating progress.
            </p>

            <p>
              They search for well-defined machine learning challenges likely to be a large climate impact if solved. For these, they collate and release data; write software tools to make it easy for people to consume this data; run collaborative &quot;global research projects&quot;, where everyone from 16-year-olds to PhD students and corporate research labs can help solve the ML task; and then once the community has developed good solutions, help to put them into production so that emissions may be reduced as soon as possible.
            </p>


            {/* <p>
              OCF is working with the UK National Grid Electricity System Operator to address this challenge, and will provide 1.5 years of satellite training data, in addition to publishing their current, state-of-the-art model&apos;s weights for us to build upon.
            </p> */}
          </div>
        </div>
      </section>

      <section className='ch-section ch-challenge-pv'>
        <div className='ch-section-content'>
          <div className='ch-section-heading'>
            <h2>
              Improving Solar Power Production Forecasts
            </h2>
          </div>
          <div className='ch-section-description'>
            <p>
              The ultimate plan is to collaborate to build the world&apos;s best near-term forecasting system for solar electricity generation for anyone to use!
            </p>

            <p>
              Better near-term forecasting of solar electricity generation will enable electricity grid operators around the world to do a better job of scheduling their grids. This, in turn, will reduce carbon emissions and reduce electricity costs for end-users.
            </p>

            <p>
              For example, National Grid in the UK need to have idle natural gas turbines on standby in the event of a sudden fall in solar power production, say, when a large cloud covers a solar farm. With better forecasting techniques, their use could be minimised, saving a significant volume of carbon emissions, potentially up to 100 kilotonnes a year.
            </p>

            <p>
              While it is incredibly difficult to accurately predict the climate impact, a rough estimate suggests that better solar power forecasts if deployed worldwide could reduce global carbon emissions by about 100 million tonnes of CO<sub>2</sub> a year by 2030.
            </p>

            <p>
              For more background and motivation, check out this <a href="https://www.wired.co.uk/article/solar-weather-forecasting">Wired article</a> about the solar electricity forecasting work of our partner, <a href="https://www.openclimatefix.org/">Open Climate Fix</a>.
            </p>
          </div>
        </div>

      </section>

      {/* <section className='ch-section ch-challenge-impact'>
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
      </section> */}

      <section className='ch-section ch-challenge-ml-task'>
        <div className='ch-section-content'>
          <div className='ch-section-heading'>
            <h2>
              The Challenge of Climate Hack.AI
            </h2>
          </div>
          <div className='ch-section-description'>
            <p>
              Building the world&apos;s best near-term forecasting system for solar electricity generation is a huge endeavour. This is where Climate Hack.AI comes in.
            </p>
            <p>
              The challenge is to build on the state of the art in forecasting the next hour of satellite imagery from the last hour of satellite imagery to eventually feed into and improve solar power prediction models.
            </p>
            <p>
              From a series of twelve 128&times;128 images cropped out of much larger satellite images taken five minutes apart, the goal is to accurately predict the centre 64&times;64 pixels of the next twelve images, corresponding to the next hour of satellite imagery.
            </p>
          </div>
        </div>
      </section>

      <section className='ch-section ch-challenge-satellite-img'>
        <div className='ch-section-content'>
          <div className='ch-section-heading'>
            <h2>
              The Data
            </h2>
          </div>
          <div>
            <p>
              <a href="https://www.openclimatefix.org/">Open Climate Fix</a> have provided just under two years of 1,843&times;891 &quot;high resolution visible&quot; satellite imagery over the UK and north-western Europe from EUMETSAT&apos;s <a href="https://www.eumetsat.int/rapid-scanning-service">Spinning Enhanced Visible and InfraRed Imager Rapid Scanning Service</a> with a spatial resolution of about 2-3 km (decreasing south to north).
            </p>
            <br />
            <img src={satelliteImage} style={{ width: '100%', backgroundColor: '#f1f5f9', borderRadius: '3px', boxSizing: 'border-box' }} />
          </div>
        </div>
      </section>
    </div>

    <Footer />

  </div>;
}
