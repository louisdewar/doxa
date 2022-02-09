import { resetPassword } from 'api/auth';
import { DoxaError } from 'api/common';
import Button from 'components/Button';
import Card from 'components/Card';
import TextBox from 'components/TextBox';
import { useMemo, useState } from 'react';
import { Link } from 'react-router-dom';

function PasswordReset() {
  return <Card>
    <h2>Password reset</h2>
    <p>Your password has been reset.</p>
    <p><Link to="/authenticate/login">Login now</Link></p>
  </Card>;
}

export default function ResetCallback() {
  const [handler, setHandler] = useState(null);
  const [error, setError] = useState(null);
  
  const [verificationCode, setVerificationCode] = useState('');
  const [newPassword, setNewPassword] = useState('');
  const queryVerificationCode = useMemo(() => {
    const queryParams = new URLSearchParams(window.location.search);
    return queryParams.get('verification_code');
  }, []);

  const submittable = (verificationCode || queryVerificationCode) && newPassword;

  const handleSubmit = async e => {
    e.preventDefault();

    if (!submittable) {
      return;
    }

    try {
      const response = await resetPassword(verificationCode || queryVerificationCode, newPassword);

      setHandler(response.incomplete(type => {
        if (type === 'password_reset') {
          return PasswordReset;
        } else {
          setError(`Unknown response type from server: ${type}`);
          return null;
        }
      }));
    } catch (e) {
      if (e instanceof DoxaError) {
        console.error(`Failed to reset password (${e.error_code}): ${e.error_message}`);
        setError(e.error_message);
      } else {
        console.error(`Failed to reset password: ${e}`);
        setError(true);
      }
    }
  };

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
        <TextBox type="text" value={verificationCode || queryVerificationCode} setValue={setVerificationCode} disabled={!!queryVerificationCode} placeholder="Verification code" /><br />
        <TextBox type="password" value={newPassword} setValue={setNewPassword} placeholder="New password" /><br />
        <br />
        <Button buttonProps={{ onClick: handleSubmit }} success disabled={!submittable}>
          Update password
        </Button>
      </form>
    </Card>
  </>;
}
