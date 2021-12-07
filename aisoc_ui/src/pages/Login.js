import { useAuth } from 'hooks/useAuth';
import { useState } from 'react';


export default function Login() {
  const auth = useAuth();

  const [username, setUsername] = useState('');
  const handleUsernameChange = e => {
    setUsername(e.target.value);
  };

  const [password, setPassword] = useState('');
  const handlePasswordChange = e => {
    setPassword(e.target.value);
  };

  const handleSubmit = async e => {
    e.preventDefault();

    if (!username || !password) return;

    await auth.login(username, password);
  };

  return <>
    Beautiful, unstyled login form: <br /><br />

    <form onSubmit={handleSubmit}>
      Username: <input type="text" value={username} onChange={handleUsernameChange} /><br />
      Password: <input type="text" value={password} onChange={handlePasswordChange} /><br />
      <input type="submit" value="Log in" />
    </form>


  </>;
}
