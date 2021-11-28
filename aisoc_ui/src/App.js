// import Landing from 'pages/Landing.js';
import ClimateHack from 'competitions/climatehack/ClimateHack';
import Uttt from 'competitions/uttt/Uttt';
import {
  BrowserRouter as Router, Redirect, Route, Switch
} from 'react-router-dom';
import './App.scss';


const COMPETITIONS = {
  climatehack: ClimateHack,
  uttt: Uttt,
};

const DEFAULT_COMPETITION = process.env.REACT_APP_DEFAULT_COMPETITION ?? 'uttt';


export default function App() {
  return (process.env.REACT_APP_MULTIPLE_COMPETITIONS != 'false') ? (
    <Router>
      <Switch>
        {Object.keys(COMPETITIONS).map(competition => (
          <Route path={`/c/${competition}/`} key={competition} component={COMPETITIONS[competition]} />
        ))}
        <Route path="/">
          <Redirect to={`/c/${DEFAULT_COMPETITION}/`} />
        </Route>
      </Switch>
    </Router>
  ) : (
    <Router>
      <Switch>
        <Route path="/">
          {(Competition => <Competition />)(COMPETITIONS[DEFAULT_COMPETITION])}
        </Route>
      </Switch>
    </Router>
  );
}
