import './SplashNavbar.scss';

export default function SplashNavbar({ baseUrl }) {
  return <nav className="ch-navbar">
    <a href={baseUrl} className='ch-navbar-title'>Climate<span>Hack</span></a>
    <a href={baseUrl}>Home</a>
    <a href="#">The Challenge</a>
    <a href="#">Our Partners</a>
    <a href={`${baseUrl}compete`} className='ch-navbar-active'>Compete on Doxa
    </a>
  </nav>;
}
