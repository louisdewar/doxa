import { AuthProvider, useAuth } from 'hooks/useAuth';
import Account from 'pages/Account';
import Home from 'pages/Home';
import Invite from 'pages/Invite';
import Login from 'pages/Login';
import Logout from 'pages/Logout';
import { lazy, Suspense } from 'react';
import {
  BrowserRouter as Router, Redirect, Route, Switch
} from 'react-router-dom';


const COMPETITIONS = {
  climatehack: lazy(() => import('competitions/climatehack/ClimateHack')),
  uttt: lazy(() => import('competitions/uttt/Uttt')),
};

const DEFAULT_COMPETITION = process.env.REACT_APP_DEFAULT_COMPETITION ?? 'uttt';


/**
 * This component only shows momentarily while the rest
 * of the UI is loading or the login status of the user
 * is in the process of being determined.
 */
function Loading() {
  return <span></span>;
}


function Routes() {
  const auth = useAuth();
  if (auth.loading) {
    return <Loading />;
  }

  const multipleCompetitionsAllowed = process.env.REACT_APP_MULTIPLE_COMPETITIONS != 'false';
  return <Router>
    <Switch>
      <Route path='/login'>
        {auth.isLoggedIn() ? <Redirect to='/' /> : <Login />}
      </Route>
      <Route path='/logout'>
        <Logout />
      </Route>
      <Route path='/account'>
        {auth.isLoggedIn() ? <Account /> : <Redirect to='/login' />}
      </Route>
      <Route path='/invite/:id'>
        <Invite />
      </Route>

      {multipleCompetitionsAllowed && Object.keys(COMPETITIONS).map(competition => (
        <Route path={`/c/${competition}/`} key={competition} component={COMPETITIONS[competition]} />
      ))}
      <Route path='/'>
        {multipleCompetitionsAllowed ? <Home />
          : (Competition => <Competition />)(COMPETITIONS[DEFAULT_COMPETITION])}
      </Route>
    </Switch>
  </Router>;
}

export default function App() {
  return <AuthProvider>
    <Suspense fallback={Loading()}>
      <Routes />
    </Suspense>
  </AuthProvider>;
}
