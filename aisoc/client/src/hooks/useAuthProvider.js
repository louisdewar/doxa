import { acceptInvite, getInviteInfo, getUserInfo, login } from 'api/auth';
import { useEffect, useState } from 'react';


/**
 * This hook manages the auth provider object and
 * handles the authentication state of the application.
 */
export function useAuthProvider() {
  const [loading, setLoading] = useState(true);
  const [user, setUser] = useState(null);
  const [authToken, setAuthToken] = useState(() => {
    const token = sessionStorage.getItem('doxa-auth-token');
    setLoading(false);
    return token;
  });

  const updateAuthToken = token => {
    if (!token) {
      setAuthToken(null);
      sessionStorage.removeItem('doxa-auth-token');
      return;
    }

    setAuthToken(token);
    sessionStorage.setItem('doxa-auth-token', token);
  };

  const refresh = async () => {
    if (!authToken && user) {
      setAuthToken(null);
      return;
    }

    try {
      const info = await getUserInfo(authToken);
      setUser({
        username: info.username,
        admin: info.admin ?? false,
        competitions: info.competitions ?? []
      });
    } catch {
      setAuthToken(null);
      setUser(null);
    }
  };

  useEffect(async () => {
    await refresh();
  }, []);

  return {
    loading,
    user,
    isLoggedIn() {
      return !!authToken;
    },
    async login(username, password) {
      updateAuthToken(await login(username, password));
      setUser({ username });
    },
    logout() {
      updateAuthToken(null);
      setUser(null);
    },
    async getInviteInfo(id) {
      return await getInviteInfo(id);
    },
    async acceptInvite(id, username, password) {
      return await acceptInvite(id, username, password);
    }
  };
}
