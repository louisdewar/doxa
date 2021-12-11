import { Route, Switch, useRouteMatch } from 'react-router-dom';
import Home from './pages/Home';


export default function ClimateHack() {
  let { path } = useRouteMatch();

  return <Switch>
    <Route path={path}>
      <Home />
    </Route>
  </Switch>;
}
