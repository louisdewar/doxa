// import Landing from 'pages/Landing.js';
import Game from 'competitions/uttt/pages/Game';
import Live from 'competitions/uttt/pages/Live';
import Match from 'competitions/uttt/pages/Match';
import User from 'competitions/uttt/pages/User.js';
import Uttt from 'competitions/uttt/pages/Uttt.js';
import {
  BrowserRouter as Router, Redirect, Route, Switch
} from 'react-router-dom';
import './App.scss';



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
