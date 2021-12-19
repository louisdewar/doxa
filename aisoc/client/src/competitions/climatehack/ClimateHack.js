import Container from 'components/Container';
import Navbar from 'components/Navbar';
import { Route, Switch, useRouteMatch } from 'react-router-dom';
import Home from './pages/Home';
import User from './pages/User';


function Layout({ children }) {
  return <>
    <Navbar competition="climatehack" competitionName="Climate Hack" />
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
    <Route path={path}>
      <Layout>
        <Home baseUrl={path} />
      </Layout>
    </Route>
  </Switch>;
}
