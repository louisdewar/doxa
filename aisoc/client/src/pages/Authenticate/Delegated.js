import { authorizeDelegatedLogin } from 'api/auth';
import { DoxaError } from 'api/common';
import Button from 'components/Button';
import Card from 'components/Card';
import TextBox from 'components/TextBox';
import { useAuth } from 'hooks/useAuth';
import { useMemo, useState } from 'react';
import { Redirect } from 'react-router-dom';
import { Link } from 'react-router-dom';

export default function Delegated() {
  const [error, setError] = useState(null);
  const [success, setSuccess] = useState(false);
  const auth = useAuth();

  const [verificationCode, setVerificationCode] = useState('');
  const queryVerificationCode = useMemo(() => {
    const queryParams = new URLSearchParams(window.location.search);
    return queryParams.get('verification_code');
  }, []);

  if (!auth.isLoggedIn()) {
    return <Redirect to="/authenticate/login" />;
  }

  // It can take a bit of time to get the user info
  if (!auth.user) {
    return null;
  }

  const handleSubmit = async e => {
    e.preventDefault();

    try {
      await authorizeDelegatedLogin(auth.token, verificationCode || queryVerificationCode);

      setSuccess(true);
    } catch (e) {
      if (e instanceof DoxaError) {
        console.error(`Failed to login (${e.error_code}): ${e.error_message}`);
        setError(e.error_message);
      } else {
        console.error(`Failed to login: ${e}`);
      }
      setError(true);
    }
  };
  
  let errorCard = null;
  if (error !== null) {
    errorCard = <Card>
      <p>Uh oh — something went wrong with the delegated login!</p>
      {typeof error === 'string'? <p>{error}</p>: null}
    </Card>;
  }

  if (success) {
    return <>
      <Card>
        <h2>Successfully authorized delegated login</h2>
        <Link to="/">
          <Button success>Return to the hompage</Button>
        </Link>
      </Card>
    </>;
  }

  return <>
    {errorCard}

    <Card>
      <Link to="/">
        <Button>Return to the hompage</Button>
      </Link>
      <h2>Authorize a delegated login for {auth.user.username}:</h2>
      <p>This will let another device authenticate as you, {auth.user.username}, so make sure you trust where you got this code from.</p>
      <form onSubmit={handleSubmit}>
        <TextBox type="text" value={verificationCode || queryVerificationCode} setValue={setVerificationCode} placeholder="Verification Code" disabled={!!queryVerificationCode} />
        <Button buttonProps={{ onClick: handleSubmit }}>
            Authorized delegated login
        </Button>
      </form>
    </Card>
  </>;
}
