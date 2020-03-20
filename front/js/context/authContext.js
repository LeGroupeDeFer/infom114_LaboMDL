import React, {
  createContext,
  useContext,
  useEffect,
  useState,
  useLayoutEffect,
  useDebugValue
} from 'react';
import api from '../lib/api';
import * as jwtDecode from 'jwt-decode';


class AuthError extends Error { }
const AuthContext = createContext();
const store = window.localStorage;


export function AuthProvider({ children }) {

  /* Internal state */
  const [user, setUser] = useState(null);
  const [token, setToken] = useState(null);
  const [loginPending, setLoginPending] = useState(null);
  const [logoutPending, setLogoutPending] = useState(null);

  useEffect(() => {
    const storedUser = store.getItem('__auth_user__');
    const storedToken = store.getItem('__auth_token__');
    if (!user && storedUser) {
      setUser(JSON.parse(storedUser));
      setToken(jwtDecode(storedToken));
    }
  });

  useEffect(() => {
    let registered = true;

    if (loginPending)
      loginPending.then(({ user, token }) => {
        if (registered) {
          setUser(user);
          setToken(jwtDecode(token));
          store.setItem('__auth_user__', JSON.stringify(user));
        }
      });

    if (logoutPending)
      logoutPending.then(_ => {
        if (registered) {
          setUser(null)
          store.removeItem('__auth_user__');
        }
      });

    return () => registered = false;
  }, [logoutPending, loginPending]);

  useDebugValue(user ? 'Connected' : 'Anonymous');

  function login(email, password) {
    if (user !== null)
      throw new AuthError('User already connected');
    let promise = api.login(email, password);
    setLoginPending(promise);
    return promise;
  }

  function logout() {
    let promise = api.logout();
    setLogoutPending(promise);
    return promise;
  }

  function register(newUser) {
    if (user !== null)
      throw new AuthError('User already connected');
    return api.register(newUser);
  }

  return (
    <AuthContext.Provider value={{ login, logout, register, user, token }}>
      {children}
    </AuthContext.Provider>
  );

}

export const useAuth = () => useContext(AuthContext);
