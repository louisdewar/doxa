import { Route, Switch, useRouteMatch } from 'react-router-dom';
import Game from './pages/Game';
import Home from './pages/Home';
import Live from './pages/Live';
import Match from './pages/Match';
import User from './pages/User';


export default function Uttt() {
  let { path } = useRouteMatch();

  return <Switch>
    <Route path={`${path}/match/:matchID/game/:gameID`}>
      <Game />
    </Route>
    <Route path={`${path}/_agent/:agentID/live`}>
      <Live />
    </Route>
    <Route path={`${path}/match/:matchID`}>
      <Match />
    </Route>
    <Route path={`${path}/user/:username`}>
      <User />
    </Route>
    <Route path={path}>
      <Home />
    </Route>
  </Switch>;
}
