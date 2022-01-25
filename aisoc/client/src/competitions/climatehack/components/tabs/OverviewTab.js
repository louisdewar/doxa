import { Link } from 'react-router-dom';
import satelliteImage from '../../assets/satellite-image.png';
import './OverviewTab.scss';


export default function OverviewTab({ baseUrl }) {
  return <div className="ch-tab ch-overview-tab">
    <h2>Overview</h2>

    <div className='ch-tab-box'>
      <p className='leading'>
        Your challenge &ndash; should you choose to accept it &ndash; is to predict the next two hours of satellite imagery from the previous hour of satellite imagery over a slightly larger area better than the current state of the art.
      </p>
    </div>

    <p>
      The ultimate end-goal is to collaborate to build the world&apos;s best near-term forecasting system for solar electricity generation, which would allow electricity system operators around the world to better schedule their grids, saving potentially ~100 million tonnes of CO<sub>2</sub> a year by 2030 if deployed worldwide.
    </p>

    <p>
      The key to this lies in developing better satellite imagery nowcasting techniques, which would allow better cloud coverage predictions, amongst other things.  This is where you come in.
    </p>

    <p>
      Read more about how this can improve solar power production forecasts and how your winning model could help reduce carbon emissions by ~100,000 tonnes a year if deployed by the National Grid Electricity System Operator on <Link to={`${baseUrl}challenge`}>the challenge page</Link>.
    </p>

    <h3>The dataset</h3>

    <p>
      <a href="https://www.openclimatefix.org/">Open Climate Fix</a> have provided &quot;high resolution visible&quot; satellite imagery taken over the UK and north-western Europe from a geostationary orbit between January 2020 and November 2021. It was sourced from EUMETSAT&apos;s <a href="https://www.eumetsat.int/rapid-scanning-service">Spinning Enhanced Visible and InfraRed Imager Rapid Scanning Service</a>. The individual images are 1,843&times;891 and have a spatial resolution of about 2-3 km (decreasing south to north). The satellite produces images every five minutes, so an hour of satellite imagery consists of 12 timesteps.
    </p>

    <img src={satelliteImage} style={{ width: '100%', backgroundColor: '#f1f5f9', borderRadius: '3px', boxSizing: 'border-box' }} />

    <h3>Your machine learning challenge</h3>

    <p className='ch-tab-box'>
      From twelve 128&times;128-pixel images taken five minutes apart (one hour of data), predict the next two hours of satellite imagery for the smaller central 64&times;64-pixel region.
      <br /><br />
      <strong>Input</strong>: an hour of satellite imagery for a 128&times;128-pixel region (<code>12 timesteps &times; 128 pixels &times; 128 pixels</code>), as well as the datetime and geospatial positions of the images (which may be useful to feed into your model).
      <br /><br />
      <strong>Output</strong>: the next two hours of satellite imagery for the 64&times;64-pixel area at the centre of the input region (<code>24 timesteps &times; 64 pixels &times; 64 pixels</code>).
    </p>
    <p>
      Note that this means the spatial extent of the input is larger than the output!
    </p>
    <p>
      For the loss function and scoring metric, Open Climate Fix recommend using the multi-scale structural similarity index measure (MS-SSIM). In their experience, MSE tends to encourage models to produce overly blurry predictions, so MS-SSIM is better. They have a PyTorch implementation of a differentiable MS-SSIM <a href="https://github.com/openclimatefix/nowcasting_utils/blob/main/nowcasting_utils/models/losses/StructuralSimilarity.py#L45">here</a>.
    </p>
    <p>
      The satellite imagery dataset includes data from all hours of the day. It is recommended that you only select &quot;daylight&quot; hours, as the submissions will be tested on &quot;daylight&quot; hours only.
    </p>
    <p>
      &quot;Daylight&quot; hours are defined as as hours where the sun is at least 10 degrees above the horizon, as measured from the centre of the 128&times;128 pixel input image. The angle of the sun can be computed using <a href="https://pvlib-python.readthedocs.io/en/stable/generated/pvlib.solarposition.get_solarposition.html">pvlib.solarposition.get_solarposition</a>
    </p>
    <p>
      Given the satellite images are 1,843&times;891 pixels, you can get a huge number of 128&times;128 training examples by randomly selecting 128&times;128 crops from the satellite imagery. You just want to make sure that the entire temporal extent of each example is in &quot;daylight&quot;, i.e. the sun is at least 10 degrees above the horizon as measured from the centre of each 128&times;128 crop.
    </p>

    <h3>Video animation</h3>

    <p>
      Take a look at this short video animation of clouds moving over the UK when aligned with a solar electricity power dataset for inspiration!
    </p>

    <iframe width="560" height="315" src="https://www.youtube-nocookie.com/embed/IOp-tj-IJpk" title="YouTube video player" frameBorder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowFullScreen></iframe>
  </div>;
}
