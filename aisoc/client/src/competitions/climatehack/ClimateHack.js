import Container from 'components/Container';
import Navbar from 'components/Navbar';
import { Route, Switch, useRouteMatch } from 'react-router-dom';
import Challenge from './pages/Challenge';
import ComingSoon from './pages/ComingSoon';
import Game from './pages/Game';
import Home from './pages/Home';
import Partners from './pages/Partners';
import Splash from './pages/Splash';
import User from './pages/User';


function Layout({ children }) {
  return <>
    <Navbar competition="climatehack" competitionName="Climate Hack.AI" />
    <Container>{children}</Container>
  </>;
}


export default function ClimateHack() {
  const { path } = useRouteMatch();

  return <Switch>
    <Route path={`${path}user/:user`}>
      <Layout>
        <User baseUrl={path} />
      </Layout>
    </Route>
    <Route path={`${path}game/:game`}>
      <Layout>
        <Game baseUrl={path} />
      </Layout>
    </Route>
    <Route path={`${path}compete`}>
      <Layout>
        <Home baseUrl={path} />
      </Layout>
    </Route>
    <Route path={`${path}challenge`}>
      <Challenge baseUrl={path} />
    </Route>
    <Route path={`${path}partners`}>
      <Partners baseUrl={path} />
    </Route>
    <Route path={`${path}comingsoon`}>
      <ComingSoon baseUrl={path} />
    </Route>
    <Route path={path}>
      <Splash baseUrl={path} />
    </Route>
  </Switch>;
}
