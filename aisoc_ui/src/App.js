// import Landing from 'pages/Landing.js';
import Uttt from 'pages/Uttt.js';
import User from 'pages/User.js';
import Match from 'pages/Match';
import Game from 'pages/Game';
import Live from 'pages/Live';

import './App.scss';

import {
  BrowserRouter as Router,
  Switch,
  Route,
  Redirect,
} from 'react-router-dom';

function App() {
  return (
    <Router>
      <Switch>
        <Route path="/c/uttt/match/:matchID/game/:gameID">
          <Game />
        </Route>
        <Route path="/c/uttt/_agent/:agentID/live">
          <Live />
        </Route>
        <Route path="/c/uttt/match/:matchID">
          <Match />
        </Route>
        <Route path="/c/uttt/user/:username">
          <User />
        </Route>
        <Route path="/c/uttt">
          <Uttt />
        </Route>
        <Route path="/">
          <Redirect to='/c/uttt' />
        </Route>
      </Switch>
    </Router>
  );
}

export default App;
