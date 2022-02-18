import Container from 'components/Container';
import Footer from 'components/Footer';
import Navbar from 'components/Navbar';
import { lazy } from 'react';
import { Redirect, Route, Switch, useRouteMatch } from 'react-router-dom';

const Challenge = lazy(() => import('./pages/Challenge'));
const Submission = lazy(() => import('./pages/Submission'));
const Home = lazy(() => import('./pages/Home'));
const Partners = lazy(() => import('./pages/Partners'));
const Splash = lazy(() => import('./pages/Splash'));
const User = lazy(() => import('./pages/User'));

function Layout({ children, baseUrl }) {
  return <div className='main-wrapper'>
    <Navbar competition="climatehack" competitionName="Climate Hack.AI" competitionLink={`${baseUrl}compete`} />
    <Container>{children}</Container>
    <Footer />
  </div>;
}

export default function ClimateHack() {
  const { path } = useRouteMatch();

  return <Switch>
    <Route path={`${path}user/:user`}>
      <Layout baseUrl={path}>
        <User baseUrl={path} />
      </Layout>
    </Route>
    <Route path={`${path}submission/:id`}>
      <Layout baseUrl={path}>
        <Submission baseUrl={path} />
      </Layout>
    </Route>
    <Route path={`${path}compete/:tab?/:subtab?`}>
      <Layout baseUrl={path}>
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
