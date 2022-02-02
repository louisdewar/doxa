import { AuthProvider, useAuth } from 'hooks/useAuth';
import Terms from 'pages/Terms';
import { lazy, Suspense } from 'react';
import {
  BrowserRouter as Router, Redirect, Route, Switch
} from 'react-router-dom';
import './App.scss';
import { COMPETITIONS, DEFAULT_COMPETITION } from './competitions';

const About = lazy(() => import('pages/About'));
const Account = lazy(() => import('pages/Account'));
const Authenticate = lazy(() => import('pages/Authenticate'));
const Error404 = lazy(() => import('pages/Error404'));
const Landing = lazy(() => import('pages/Landing'));
const Logout = lazy(() => import('pages/Logout'));



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
      <Route path='/about'>
        <About />
      </Route>
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
