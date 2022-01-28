import { authorizeDelegatedLogin } from 'api/auth';
import { DoxaError } from 'api/common';
import Button from 'components/Button';
import Card from 'components/Card';
import TextBox from 'components/TextBox';
import { useAuth } from 'hooks/useAuth';
import { useMemo, useState } from 'react';
import { Link, Redirect, useHistory } from 'react-router-dom';

export default function Delegated() {
  const [error, setError] = useState(null);
  const auth = useAuth();
  const history = useHistory();

  const [verificationCode, setVerificationCode] = useState('');
  const queryVerificationCode = useMemo(() => {
    const queryParams = new URLSearchParams(window.location.search);
    return queryParams.get('verification_code');
  }, []);

  if (!auth.isLoggedIn()) {
    // TODO: There is probably a better way of doing this:
    const redirectURL = window.location.href.substring(window.location.origin.length);
    return <Redirect to={`/authenticate/login?post_login_redirect=${encodeURIComponent(redirectURL)}`} />;
  }

  // It can take a bit of time to get the user info
  if (!auth.user) {
    return null;
  }

  const handleSubmit = async e => {
    e.preventDefault();

    try {
      await authorizeDelegatedLogin(auth.token, verificationCode || queryVerificationCode);

      history.push('/authenticate/delegated/success');
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

  return <>
    {error !== null && <Card>
      <p>Uh oh — something went wrong with the delegated login!</p>
      {typeof error === 'string' ? <p>{error}</p> : null}
    </Card>}

    <Card>
      <h2>Authorise a delegated login</h2>
      <p>
        Hi {auth.user.username},
      </p>
      <p>
        An application is requesting to authenticate as you on your behalf.
      </p>
      <p>
        If you do not trust the source of the code below, do not authorise the login attempt.
      </p>

      <form onSubmit={handleSubmit}>
        <TextBox type="text" value={verificationCode || queryVerificationCode} setValue={setVerificationCode} placeholder="Verification Code" disabled={!!queryVerificationCode} />
        <Button success buttonProps={{ onClick: handleSubmit }}>
          Authorise login
        </Button>
        <Link to="/">
          <Button>Return to the hompage</Button>
        </Link>
      </form>
    </Card>
  </>;
}
