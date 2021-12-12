import { Route, Switch, useRouteMatch } from 'react-router-dom';
import Home from './pages/Home';
import User from './pages/User';


export default function ClimateHack() {
  const { path } = useRouteMatch();

  return <Switch>
    <Route path={`${path}user/:user`}>
      <User baseUrl={path} />
    </Route>
    <Route path={path}>
      <Home baseUrl={path} />
    </Route>
  </Switch>;
}
