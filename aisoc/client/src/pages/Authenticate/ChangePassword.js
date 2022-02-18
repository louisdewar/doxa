import { changePassword } from 'api/auth';
import { DoxaError } from 'api/common';
import Button from 'components/Button';
import Card from 'components/Card';
import TextBox from 'components/TextBox';
import { useAuth } from 'hooks/useAuth';
import { useState } from 'react';
import { Link, Redirect } from 'react-router-dom';

function PasswordChanged() {
  return <Card>
    <h2>Password changed</h2>
    <p>Your password has been updated.</p>
    <p><Link to="/account">Go to account</Link></p>
  </Card>;
}

export default function ResetCallback() {
  const [handler, setHandler] = useState(null);
  const [error, setError] = useState(null);
  const auth = useAuth();


  const [currentPassword, setCurrentPassword] = useState('');
  const [newPassword, setNewPassword] = useState('');

  const submittable = currentPassword && newPassword && newPassword.length >= 8;

  const handleSubmit = async e => {
    e.preventDefault();

    if (!submittable) {
      return;
    }

    try {
      const response = await changePassword(auth.token, currentPassword, newPassword);

      setHandler(response.incomplete(type => {
        if (type === 'password_changed') {
          return PasswordChanged;
        } else {
          setError(`Unknown response type from server: ${type}`);
          return null;
        }
      }));
    } catch (e) {
      if (e instanceof DoxaError) {
        console.error(`Failed to update password (${e.error_code}): ${e.error_message}`);
        setError(e.error_message);
      } else {
        console.error(`Failed to update password: ${e}`);
        setError(true);
      }
    }
  };

  if (!auth.isLoggedIn()) {
    return <Redirect to="/authenticate/login" />;
  }

  if (handler) {
    return <>{handler}</>;
  }

  return <>
    {error && <Card>
      <p>Sorry &ndash; we were unable to change your password!</p>
      {typeof error === 'string' ? <p>{error}</p> : null}
    </Card>}

    <Card>
      <h1>Change your password</h1>
      <form onSubmit={handleSubmit}>
        <TextBox type="password" value={currentPassword} setValue={setCurrentPassword} placeholder="Current password" /><br />
        <TextBox type="password" value={newPassword} setValue={setNewPassword} placeholder="New password" /><br />
        <br />
        <Button buttonProps={{ onClick: handleSubmit }} success disabled={!submittable}>
          Update password
        </Button>
      </form>
    </Card>
  </>;
}
