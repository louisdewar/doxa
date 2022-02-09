import { login } from 'api/auth';
import { DoxaError } from 'api/common';
import Button from 'components/Button';
import Card from 'components/Card';
import TextBox from 'components/TextBox';
import { useAuth } from 'hooks/useAuth';
import { useState } from 'react';
import { Link, Redirect, useHistory } from 'react-router-dom';


const ERROR_MESSAGES = {
  'INCORRECT_CREDENTIALS': 'Your username and/or password are incorrect.'
};

export default function Login({ postLoginRedirect }) {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [handler, setHandler] = useState(null);

  const [error, setError] = useState(null);

  const auth = useAuth();
  const history = useHistory();

  const handleSubmit = async e => {
    e.preventDefault();

    if (!username || !password) return;

    try {
      const response = await login(username, password);

      const authToken = response.authToken();

      if (authToken) {
        auth.setAuthToken(authToken);

        if (postLoginRedirect) {
          history.push(auth.consumePostLoginRedirectUrl());
        } else {
          history.push('/#login-success');
        }
      } else {
        setHandler(response.incomplete());
      }
    } catch (e) {
      if (e instanceof DoxaError) {
        console.error(`Failed to login (${e.error_code}): ${e.error_message}`);
        setError(ERROR_MESSAGES[e.error_code] || e.error_message);
      } else {
        console.error(`Failed to login: ${e}`);
        setError(true);
      }
    }
  };

  if (auth.isLoggedIn()) {
    return <Redirect to="/" />;
  }

  if (handler) {
    return <>{handler}</>;
  }

  return <>
    {error && <Card>
      <p>Sorry &ndash; we could not log you in with those credentials! Double-check and try again.</p>
      {typeof error === 'string' ? <p>{error}</p> : null}
    </Card>}

    <Card>
      <h1>Login</h1>
      <form onSubmit={handleSubmit}>
        <TextBox type="text" value={username} setValue={setUsername} placeholder="Username or email" /><br />
        <TextBox type="password" value={password} setValue={setPassword} placeholder="Password" /><br />
        <Link to="/authenticate/forgot_password"><p style={{ marginTop: '0' }}>Forgot password?</p></Link>
        <Button success buttonProps={{ onClick: handleSubmit }} disabled={!username || !password}>
          Log in
        </Button>
        <span style={{ marginLeft: '1rem' }}>
          Don&apos;t have an account? <Link to="/authenticate/register">Register</Link>.
        </span>
      </form>
    </Card>
  </>;
}
