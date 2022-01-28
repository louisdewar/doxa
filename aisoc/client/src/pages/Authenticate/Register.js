import { register } from 'api/auth';
import { DoxaError } from 'api/common';
import Button from 'components/Button';
import Card from 'components/Card';
import TextBox from 'components/TextBox';
import { useAuth } from 'hooks/useAuth';
import { useState } from 'react';
import { Link, Redirect, useHistory } from 'react-router-dom';



export default function Register() {
  // TODO: validate
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [email, setEmail] = useState('');
  const [handler, setHandler] = useState(null);
  const [error, setError] = useState(null);
  const auth = useAuth();
  const history = useHistory();

  const submittable = username && password && email && username.length >= 3 && password.length >= 8 && email.length >= 3;

  const handleSubmit = async e => {
    e.preventDefault();

    if (!username || !password || !submittable) return;

    try {
      const response = await register(username, email, password);
      console.log(response);

      const authToken = response.authToken();

      if (authToken) {
        auth.setAuthToken(authToken);

        // If instead we want to show a message that says "Welcome to DOXA, [Go to account]"
        // then have a useState to set a completeMessage
        history.push('/#login-success');
      } else {
        setHandler(response.incomplete());
      }
    } catch (e) {
      if (e instanceof DoxaError) {
        console.error(`Failed to register (${e.error_code}): ${e.error_message}`);
        setError(e.error_message);
      } else {
        console.error(`Failed to register: ${e}`);
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
      <p>Sorry &ndash; we could not register you! Double-check and try again.</p>
      {typeof error === 'string' ? <p>{error}</p> : null}
    </Card>}

    <Card>
      <h1>Create a new account</h1>
      <form onSubmit={handleSubmit}>
        <TextBox type="email" value={email} setValue={setEmail} placeholder="University email" /><br />
        <TextBox type="text" value={username} setValue={setUsername} placeholder="Username" /><br />
        <TextBox type="password" value={password} setValue={setPassword} placeholder="Password" style={{ marginBottom: '5px' }} /><br />
        {/* <p>
          By registering, you agree to our <Link to="/terms" target="_blank" rel="noopener noreferrer">terms and conditions</Link>.
        </p> */}
        <Button buttonProps={{ onClick: handleSubmit }} success disabled={!submittable}>
          Register
        </Button>
        <span style={{ marginLeft: '1rem' }}>Already have an account? <Link to="/authenticate/login">Login</Link></span>
      </form>
    </Card>
  </>;
}
