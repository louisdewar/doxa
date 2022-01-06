import './Footer.scss';

export default function Footer() {
  return <footer className='ch-footer'>
    <div className='ch-footer-contact-us'>
      <a href="mailto:jamie.weigold.19@ucl.ac.uk">Contact us</a>
    </div>
    <div className='ch-footer-copyright'>
      Copyright &copy; {new Date().getFullYear()} <a href="https://uclaisociety.co.uk/">UCL AI Society</a>
    </div>
  </footer>;
}
