import Container from 'components/Container';
import Navbar from 'components/Navbar';
import { Route, Switch, useRouteMatch } from 'react-router-dom';
import Game from './pages/Game';
import Home from './pages/Home';
import Match from './pages/Match';
import User from './pages/User';


function Layout({ children }) {
  return <>
    <Navbar competition="uttt" competitionName="Ultimate Tic-Tac-Toe" />
    <Container>{children}</Container>
  </>;
}


export default function Uttt() {
  const { path } = useRouteMatch();

  return <Switch>
    <Route path={`${path}user/:user`}>
      <Layout>
        <User baseUrl={path} />
      </Layout>
    </Route>
    <Route path={`${path}match/:matchID/game/:gameID`}>
      <Layout>
        <Game baseUrl={path} />
      </Layout>
    </Route>
    <Route path={`${path}match/:id`}>
      <Layout>
        <Match baseUrl={path} />
      </Layout>
    </Route>
    <Route path={path}>
      <Layout>
        <Home baseUrl={path} />
      </Layout>
    </Route>
  </Switch>;
}
