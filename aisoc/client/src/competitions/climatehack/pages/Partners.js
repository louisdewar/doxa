import ennovate from '../assets/ennovate.png';
import newcross from '../assets/newcross-white-orange.png';
import Footer from '../components/Footer';
import SplashNavbar from '../components/SplashNavbar';
import './Partners.scss';

export default function Partners({ baseUrl }) {
  return <div className='ch-wrapper'>
    <SplashNavbar baseUrl={baseUrl} />

    <header className='ch-partners-header'>
      <h2>
        Our
      </h2>
      <h1>
        Partners
      </h1>
    </header>

    <div className='ch-panel-container'>
      <section className='ch-section ch-partners-sponsor-1'>
        <div className='ch-section-content'>
          <div className='ch-section-heading'>
            <a href="https://entaingroup.com/newsrelease/entain-launches-global-innovation-hub-ennovate/"><img src={ennovate} alt="Ennovate logo" /></a>
          </div>
          <div>

          </div>
        </div>
      </section>

      <section className='ch-section ch-partners-sponsor-2'>
        <div className='ch-section-content'>
          <div className='ch-section-heading'>
            {/* <h2>Newcross Healthcare</h2> */}
            <a href="https://www.newcrosshealthcare.com/"><img src={newcross} alt="Newcross Healthcare logo" /></a>
          </div>
          <div>
            <p>
              {'“'}At Newcross Healthcare, we&apos;re creating a local, national &ndash; and ultimately &ndash; global healthcare ecosystem. One that&apos;s modern, joined-up, efficient, reliable and most of all, effective.
            </p>
            <p>
              {'“'}The future of healthcare will be driven by technology, but never forget the human experience. Delivering more human-centred care in people&apos;s homes, technology such as wearable sensors and monitors will be more common. In addition, the advancements in technology and improvements in connectivity will deliver more responsive, tailored support in a way that improves the lives of those in our care.
            </p>
            <p>
              {'“'}With increased reliance on technology comes increased energy dependence - which is why we&apos;re supporting this ground-breaking initiative to use Artificial Intelligence to re-think the power grid to make sure that the future of healthcare is sustainable.
            </p>
            <p>
              {'“'}Moreover, the deployment of tailored renewable energy solutions could positively transform socio-economic outcomes addressing several goals across the sustainable development spectrum. For example, the World Health Organisation estimates that at least half of the world&apos;s population currently lacks essential health services. While the issue is complex, a clear opportunity exists to contribute by providing reliable, cost-effective renewable energy.
            </p>
            <p>
              {'“'}At Newcross Healthcare, we are building something to serve people like never before, while treating caregivers with the utmost respect. Together our ongoing investment in technology, clinical governance, and staff development, we will enable the highest standards of practice to deliver outstanding care to those who need it. That&apos;s how we believe we can help Britain get the care service it deserves.{'”'}
            </p>
          </div>
        </div>
      </section>
    </div>

    <Footer />

  </div>;
}
