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

  return {
    loading,
    user,
    login(username, password) {
      setAuthToken(login(username, password));
      setUser({ ...user, username });
    }
  };
}
