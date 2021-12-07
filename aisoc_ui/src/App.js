// import Landing from 'pages/Landing.js';
import ClimateHack from 'competitions/climatehack/ClimateHack';
import Uttt from 'competitions/uttt/Uttt';
import { AuthProvider, useAuth } from 'hooks/useAuth';
import { Suspense } from 'react';
import {
  BrowserRouter as Router, Redirect, Route, Switch
} from 'react-router-dom';
import './App.scss';


const COMPETITIONS = {
  climatehack: ClimateHack,
  uttt: Uttt,
};

const DEFAULT_COMPETITION = process.env.REACT_APP_DEFAULT_COMPETITION ?? 'uttt';


/**
 * This component only shows momentarily while the rest
 * of the UI is loading or the login status of the user
 * is in the process of being determined.
 */
function Loading() {
  return <span>Loading...</span>;
}


function Routes() {
  const auth = useAuth();
  if (auth.loading) {
    return <Loading />;
  }

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

export default function App() {
  return <AuthProvider>
    <Suspense fallback={Loading()}>
      <Routes />
    </Suspense>
  </AuthProvider>;
}
