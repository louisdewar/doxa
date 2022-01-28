import Container from 'components/Container';
import Navbar from 'components/Navbar';
import { useMemo } from 'react';
import { Redirect, Route, Switch, useRouteMatch } from 'react-router-dom';
import Delegated from './Delegated';
import DelegatedSuccess from './DelegatedSuccess';
import Login from './Login';
import Register from './Register';
import VerifyEmail from './VerifyEmail';



export default function Authenticate() {
  const { path } = useRouteMatch();

  // TODO: consider having authenticate provide some kind of context object to each of these which can be called with things like "login(authToken)" to automatically redirect if needed.
  // Note currently post login redirect is only handled by login (even though any authenticate provider route could authenticate)
  // and it is only set by the delegated route
  const postLoginRedirect = useMemo(() => {
    const queryParams = new URLSearchParams(window.location.search);
    return queryParams.get('post_login_redirect');
  }, [window.location.search]);

  return <>
    <Navbar />
    <Container>
      <Switch>
        <Route path={`${path}/verifyemail`}>
          <VerifyEmail postLoginRedirect={postLoginRedirect} />
        </Route>
        <Route path={`${path}/login`}>
          <Login postLoginRedirect={postLoginRedirect} />
        </Route>
        <Route path={`${path}/register`}>
          <Register postLoginRedirect={postLoginRedirect} />
        </Route>
        <Route path={`${path}/delegated/success`}>
          <DelegatedSuccess />
        </Route>
        <Route path={`${path}/delegated`}>
          <Delegated postLoginRedirect={postLoginRedirect} />
        </Route>
        <Route path={path}>
          <Redirect to={`${path}/login`} />
        </Route>
      </Switch>
    </Container>

  </>;
}
