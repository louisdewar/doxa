import { Route, Switch, useRouteMatch } from 'react-router-dom';


export default function ClimateHack() {
  let { path } = useRouteMatch();

  return <Switch>
    <Route path={path}>
      Hi!
    </Route>
  </Switch>;
}
