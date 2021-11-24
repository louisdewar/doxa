import Navbar from 'components/NavBar';

export default function Layout({ children }) {
  return <>
    <Navbar competitionName='Ultimate Tic-Tac-Toe' homepageUrl='/c/uttt/' />
    <div className="maxwidth">
      {children}
    </div>
  </>;
}
