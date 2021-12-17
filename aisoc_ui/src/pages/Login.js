import Button from 'components/Button';
import Card from 'components/Card';
import Container from 'components/Container';
import Navbar from 'components/Navbar';
import TextBox from 'components/TextBox';
import { useAuth } from 'hooks/useAuth';
import { useState } from 'react';
import { Redirect } from 'react-router-dom';


export default function Login() {
  const auth = useAuth();
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [showError, setShowError] = useState(false);

  const handleSubmit = async e => {
    e.preventDefault();

    if (!username || !password) return;

    try {
      await auth.login(username, password);
    } catch {
      setShowError(true);
    }
  };

  if (auth.isLoggedIn()) {
    return <Redirect to="/" />;
  }

  return <>
    <Navbar />
    <Container>
      {showError && <Card>
        Sorry — we could not log you in with those credentials! Double-check and try again.
      </Card>}

      <Card>
        <h1>Login</h1>
        <form>
          <TextBox type="text" value={username} setValue={setUsername} placeholder="Username" /><br />
          <TextBox type="password" value={password} setValue={setPassword} placeholder="Password" /><br />
          <Button buttonProps={{ onClick: handleSubmit }}>
            Log in
          </Button>
        </form>
      </Card>
    </Container>
  </>;
}
