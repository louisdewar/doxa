import Button from 'components/Button';
import Card from 'components/Card';
import Container from 'components/Container';
import Navbar from 'components/Navbar';
import TextBox from 'components/TextBox';
import { useAuth } from 'hooks/useAuth';
import { useEffect, useState } from 'react';
import { Link, useHistory, useParams } from 'react-router-dom';


export default function Invite() {
  const auth = useAuth();
  const history = useHistory();
  const { id } = useParams();
  const [invite, setInvite] = useState(null);
  const [notFound, setNotFound] = useState(false);
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [showError, setShowError] = useState(false);

  const handleAccept = async e => {
    e.preventDefault();
    if (password.length < 8) {
      return;
    }

    try {
      await auth.acceptInvite(id, username, password);
      history.push('/');
    } catch {
      setShowError(true);
    }
  };

  useEffect(async () => {
    try {
      const inviteInfo = await auth.getInviteInfo(id);
      setInvite(inviteInfo);
      setUsername(inviteInfo.username);
    } catch {
      setNotFound(true);
    }
  }, [setInvite]);

  if (notFound) {
    return <>
      <Navbar />
      <Container>
        <Card>
          Sorry — we could not find that invite token.
        </Card>

        <Link to="/">
          <Button>Return to the hompage</Button>
        </Link>
      </Container>
    </>;
  }

  if (!invite) {
    return <>
      <Navbar />
      <Container>
        <Card>
          Sorry — something has gone wrong!
        </Card>

        <Link to="/">
          <Button>Return to the hompage</Button>
        </Link>
      </Container>
    </>;
  }

  return <>
    <Navbar />
    <Container>
      {showError && <Card>
        Sorry — we could not process the invite acceptance at this time.
      </Card>}

      <Card>
        <h1>Invitation</h1>
        <p>
          Hi,
        </p>
        <p>
          You have been invited to join Doxa.
        </p>

        {invite.expires_at && <><strong>Invitation expiration time: </strong>{new Date(invite.expires_at).toLocaleString()}<br /><br /></>}
        {invite.enrollments && invite.enrollments.length > 0 && <><strong>Competition enrolments: </strong>{invite.enrollments && invite.enrollments.join(', ')}<br /><br /></>}

        <form onSubmit={handleAccept}>
          <TextBox
            value={username}
            setValue={setUsername}
            placeholder="Username"
            type="text"
            disabled={!!invite.username}
          />

          <TextBox
            value={password}
            setValue={setPassword}
            placeholder="Password (minimum 8 characters)"
            type="password"
          />

          <Button
            buttonProps={{
              onClick: handleAccept
            }}
            success
            disabled={password.length < 8}
          >Accept invite & join</Button>

          <Link to="/">
            <Button failure>Return to the hompage</Button>
          </Link>
        </form>
      </Card>
    </Container>



  </>;
}
