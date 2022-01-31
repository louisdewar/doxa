import Container from 'components/Container';
import Navbar from 'components/Navbar';
import { lazy } from 'react';
import { Redirect, Route, Switch, useRouteMatch } from 'react-router-dom';

const Challenge = lazy(() => import('./pages/Challenge'));
const Game = lazy(() => import('./pages/Game'));
const Home = lazy(() => import('./pages/Home'));
const Partners = lazy(() => import('./pages/Partners'));
const Splash = lazy(() => import('./pages/Splash'));
const User = lazy(() => import('./pages/User'));

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
      <Redirect to={`${path}compete`} />
    </Route>
    <Route path={path}>
      <Splash baseUrl={path} />
    </Route>
  </Switch>;
}
