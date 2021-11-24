// import Landing from 'pages/Landing.js';
import Uttt from 'competitions/uttt/Uttt';
import {
  BrowserRouter as Router, Redirect, Route, Switch
} from 'react-router-dom';
import './App.scss';



function App() {
  return (
    <Router>
      <Switch>
        <Route path="/c/uttt">
          <Uttt />
        </Route>
        <Route path="/">
          <Redirect to='/c/uttt' />
        </Route>
      </Switch>
    </Router>
  );
}

export default App;
