import { useAuth } from 'hooks/useAuth';
import { useEffect, useState } from 'react';
import { useHistory, useParams } from 'react-router-dom';


export default function Invite() {
  const auth = useAuth();
  const history = useHistory();
  const { id } = useParams();
  const [invite, setInvite] = useState(null);
  const [notFound, setNotFound] = useState(false);

  const [username, setUsername] = useState('');
  const handleUsernameChange = e => {
    setUsername(e.target.value);
  };

  const [password, setPassword] = useState('');
  const handlePasswordChange = e => {
    setPassword(e.target.value);
  };

  const handleAccept = async e => {
    e.preventDefault();
    await auth.acceptInvite(id, username, password);
    // TODO: handle error
    history.push('/');
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
    return <>Invite not found.</>;
  }

  if (!invite) {
    // TODO: display an error here
    return <></>;
  }

  return <>
    <strong>Invite ID: </strong>{id}<br /><br />
    <strong>Username: </strong>{invite.username}<br /><br />
    <strong>Expiration time: </strong>{invite.expires_at}<br /><br />
    <strong>Enrollments: </strong>{invite.enrollments && invite.enrollments.join(', ')}<br /><br />

    <form onSubmit={handleAccept}>
      Username: <input type="text" value={username} onChange={handleUsernameChange} disabled={!!invite.username} /><br />
      Password: <input type="text" value={password} onChange={handlePasswordChange} /><br />
      <input type="submit" value="Accept" />
    </form>
  </>;
}
