import { AuthProvider, useAuth } from 'hooks/useAuth';
import Account from 'pages/Account';
import Authenticate from 'pages/Authenticate';
import Error404 from 'pages/Error404';
import Landing from 'pages/Landing';
import Logout from 'pages/Logout';
import Terms from 'pages/Terms';
import { Suspense } from 'react';
import {
  BrowserRouter as Router, Redirect, Route, Switch
} from 'react-router-dom';
import './App.scss';
import { COMPETITIONS, DEFAULT_COMPETITION } from './competitions';





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
        <Redirect to='/authenticate/login' />
      </Route>
      <Route path='/logout'>
        <Logout />
      </Route>
      <Route path='/account'>
        {auth.isLoggedIn() ? <Account multipleCompetitionsAllowed={multipleCompetitionsAllowed} /> : <Redirect to='/login' />}
      </Route>
      <Route path='/authenticate'>
        <Authenticate />
      </Route>
      <Route path='/terms'>
        <Terms />
      </Route>

      {multipleCompetitionsAllowed
        ? Object.keys(COMPETITIONS).map(competition => (
          <Route path={`/c/${competition}/`} key={competition} component={COMPETITIONS[competition].competition} />
        ))
        : <Route path={`/c/${DEFAULT_COMPETITION}/`}>
          <Redirect to='/' />
        </Route>}
      {multipleCompetitionsAllowed ? <Route exact path='/'>
        <Landing competitions={COMPETITIONS} />
      </Route> : <Route path='/'>
        {(Competition => <Competition />)(COMPETITIONS[DEFAULT_COMPETITION].competition)}
      </Route>}
      <Route>
        <Error404 />
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
