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
    const token = localStorage.getItem('doxa-auth-token');
    setLoading(false);
    return token;
  });

  const updateAuthToken = token => {
    if (!token) {
      setAuthToken(null);
      localStorage.removeItem('doxa-auth-token');
      return;
    }

    setAuthToken(token);
    localStorage.setItem('doxa-auth-token', token);
  };

  const refresh = async token => {
    if (!token) {
      setUser(null);
      return;
    }

    try {
      const info = await getUserInfo(token);
      setUser({
        username: info.username,
        admin: info.admin ?? false,
        competitions: info.competitions ?? []
      });
    } catch {
      updateAuthToken(null);
      setUser(null);
    }
  };

  useEffect(async () => {
    await refresh(authToken);
  }, []);

  return {
    loading,
    user,
    isLoggedIn() {
      return !!authToken;
    },
    async login(username, password) {
      const token = await login(username, password);
      updateAuthToken(token);
      refresh(token);
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
    },
    token: authToken
  };
}
