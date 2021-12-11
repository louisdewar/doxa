import { Route, Switch, useRouteMatch } from 'react-router-dom';
import Game from './pages/Game';
import Home from './pages/Home';
import Live from './pages/Live';
import Match from './pages/Match';
import User from './pages/User';
import './Uttt.scss';


export default function Uttt() {
  let { path } = useRouteMatch();
  return <Switch>
    <Route path={`${path}match/:matchID/game/:gameID`}>
      <Game competitionBaseUrl={path} />
    </Route>
    <Route path={`${path}_agent/:agentID/live`}>
      <Live competitionBaseUrl={path} />
    </Route>
    <Route path={`${path}match/:matchID`}>
      <Match competitionBaseUrl={path} />
    </Route>
    <Route path={`${path}user/:username`}>
      <User competitionBaseUrl={path} />
    </Route>
    <Route path={path}>
      <Home competitionBaseUrl={path} />
    </Route>
  </Switch>;
}
