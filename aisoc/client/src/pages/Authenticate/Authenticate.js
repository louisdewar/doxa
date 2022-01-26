import Login from './Login';
import Container from 'components/Container';
import Navbar from 'components/Navbar';

import { Redirect, Route, Switch, useRouteMatch } from 'react-router-dom';
import VerifyEmail from './VerifyEmail';
import Register from './Register';
import Delegated from './Delegated';


export default function Authenticate() {
  const { path } = useRouteMatch();

  return <>
    <Navbar />
    <Container>
      <Switch>
        <Route path={`${path}/verifyemail`}>
          <VerifyEmail />
        </Route>
        <Route path={`${path}/login`}>
          <Login />
        </Route>
        <Route path={`${path}/register`}>
          <Register />
        </Route>
        <Route path={`${path}/delegated`}>
          <Delegated />
        </Route>
        <Route path={path}>
          <Redirect to={`${path}/login`} />
        </Route>
      </Switch>
    </Container>
    
  </>;
}