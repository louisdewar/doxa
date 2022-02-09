import Container from 'components/Container';
import Footer from 'components/Footer';
import Navbar from 'components/Navbar';
import { useAuth } from 'hooks/useAuth';
import { Redirect, Route, Switch, useRouteMatch } from 'react-router-dom';
import Delegated from './Delegated';
import DelegatedSuccess from './DelegatedSuccess';
import Login from './Login';
import Register from './Register';
import VerifyEmail from './VerifyEmail';
import ForgotPassword from './ForgotPassword';
import ResetCallback from './ResetCallback';
import ChangePassword from './ChangePassword';


export default function Authenticate() {
  const { path } = useRouteMatch();
  const auth = useAuth();

  // TODO: consider having authenticate provide some kind of context object to each of these which can be called with things like "login(authToken)" to automatically redirect if needed.
  // Note currently post login redirect is only handled by login (even though any authenticate provider route could authenticate)
  // and it is only set by the delegated route

  return <div className='main-wrapper'>
    <Navbar />
    <Container>
      <Switch>
        <Route path={`${path}/verifyemail`}>
          <VerifyEmail postLoginRedirect={auth.postLoginRedirectUrl} />
        </Route>
        <Route path={`${path}/login`}>
          <Login postLoginRedirect={auth.postLoginRedirectUrl} />
        </Route>
        <Route path={`${path}/register`}>
          <Register postLoginRedirect={auth.postLoginRedirectUrl} />
        </Route>
        <Route path={`${path}/forgot_password`}>
          <ForgotPassword postLoginRedirect={auth.postLoginRedirectUrl} />
        </Route>
        <Route path={`${path}/reset_callback`}>
          <ResetCallback postLoginRedirect={auth.postLoginRedirectUrl} />
        </Route>
        <Route path={`${path}/change_password`}>
          <ChangePassword postLoginRedirect={auth.postLoginRedirectUrl} />
        </Route>
        <Route path={`${path}/delegated/success`}>
          <DelegatedSuccess />
        </Route>
        <Route path={`${path}/delegated`}>
          <Delegated postLoginRedirect={auth.postLoginRedirectUrl} />
        </Route>
        <Route path={path}>
          <Redirect to={`${path}/login`} />
        </Route>
      </Switch>
    </Container>
    <Footer />
  </div>;
}
