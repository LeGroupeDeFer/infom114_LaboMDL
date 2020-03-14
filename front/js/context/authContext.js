import React, {
  createContext,
  useContext,
  useEffect,
  useState,
  useLayoutEffect,
  useDebugValue
} from 'react';
import api from '../lib/api';


class AuthError extends Error { }
const AuthContext = createContext();
const store = window.localStorage;


export function AuthProvider({ children }) {

  /* Internal state */
  const [user, setUser] = useState(null);

  useEffect(() => {
    const storedUser = store.getItem('__auth_user__');
    if (!user && storedUser)
      setUser(JSON.parse(storedUser));
  });

  useEffect(() => {
    if (user)
      store.setItem('__auth_user__', JSON.stringify(user));
  }, [user]);

  useDebugValue(user ? 'Connected' : 'Anonymous');

  function login(email, password) {
    if (user !== null)
      throw new AuthError('User already connected');
    return api.login(email, password).then(setUser);
  }

  function logout() {
    return api.logout()
      .then(_ => setUser(null))
      .then(_ => store.removeItem('__auth_user__'));
  }

  function register(newUser) {
    if (user !== null)
      throw new AuthError('User already connected');
    return api.register(newUser);
  }

  return (
    <AuthContext.Provider value={{ login, logout, register, user }}>
      {children}
    </AuthContext.Provider>
  );

}

export const useAuth = () => useContext(AuthContext);
