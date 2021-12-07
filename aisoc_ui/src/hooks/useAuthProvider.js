import { login } from 'api/auth';
import { useState } from 'react';


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
    if (!token) return;

    setAuthToken(token);
    sessionStorage.setItem('doxa-auth-token', token);
  };

  return {
    loading,
    user,
    isLoggedIn() {
      return !!authToken;
    },
    async login(username, password) {
      updateAuthToken(await login(username, password));
      setUser({ username });
    }
  };
}
