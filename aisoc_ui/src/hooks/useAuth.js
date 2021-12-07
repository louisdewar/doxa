import { createContext, useContext } from 'react';
import { useAuthProvider } from './useAuthProvider';


const AuthContext = createContext();

/**
 * This is a hook to allow a child element to access the auth context object
 * and re-render on any updates.
 */
export function useAuth() {
  return useContext(AuthContext);
}

/**
 * This provider makes the auth context object accessible
 * to the provider's children.
 */
export function AuthProvider({ children }) {
  return <AuthContext.Provider value={useAuthProvider()}>
    {children}
  </AuthContext.Provider>;
}
