import { requestResetPassword } from 'api/auth';
import { DoxaError } from 'api/common';
import Button from 'components/Button';
import Card from 'components/Card';
import TextBox from 'components/TextBox';
import { useAuth } from 'hooks/useAuth';
import { useState } from 'react';
import { Link, Redirect } from 'react-router-dom';

function ResetEmailSent() {
  return <Card>
    <h2>Reset email sent</h2>
    <p>
      A reset email has been sent to that account if it exists.
    </p>
    <p>
      <Link to="/">Go back home</Link>
    </p>
  </Card>;
}

export default function ForgotPassword() {
  const [email, setEmail] = useState('');
  const [handler, setHandler] = useState(null);
  const [error, setError] = useState(null);
  const auth = useAuth();

  const submittable = email && email.length >= 3;

  const handleSubmit = async e => {
    e.preventDefault();

    if (!submittable) return;

    try {
      const response = await requestResetPassword(email);

      setHandler(response.incomplete(type => {
        if (type === 'reset_password_email_sent') {
          return ResetEmailSent;
        } else {
          setError(`Unknown response type from server: ${type}`);
          return null;
        }
      }));
    } catch (e) {
      if (e instanceof DoxaError) {
        console.error(`Failed to request password reset (${e.error_code}): ${e.error_message}`);
        setError(e.error_message);
      } else {
        console.error(`Failed to request password reset: ${e}`);
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
      <p>Sorry &ndash; we were unable to reset your password!</p>
      {typeof error === 'string' ? <p>{error}</p> : null}
    </Card>}

    <Card>
      <h1>Reset your password</h1>
      <form onSubmit={handleSubmit}>
        <TextBox type="email" value={email} setValue={setEmail} placeholder="Account email" /><br />
        <Button buttonProps={{ onClick: handleSubmit }} success disabled={!submittable}>
          Send email
        </Button>
        <span style={{ marginLeft: '1rem' }}>Remembered your password? <Link to="/authenticate/login">Login</Link>.</span>
      </form>
    </Card>
  </>;
}
