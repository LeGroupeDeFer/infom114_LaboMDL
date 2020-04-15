import React, {
  createContext,
  useContext,
  useEffect,
  useState,
  useLayoutEffect,
  useDebugValue
} from 'react';
import api from '../lib/api';
import jwtDecode from 'jwt-decode';

class AuthError extends Error { }
const AuthContext = createContext();
const store = window.localStorage;


export function AuthProvider({ children }) {

  /* Internal state */
  const [user, setUser] = useState(null);
  const [token, setToken] = useState(null);
  const [request, setRequest] = useState(null);


  function login(email, password) {
    if (user !== null)
      throw new AuthError('User already connected');
    let promise = api.auth.login(email, password);
    setRequest({ type: 'login', promise });
    return promise;
  }

  function logout() {
    let promise = api.auth.logout();
    setRequest({ type: 'logout', promise });
    return promise;
  }

  function register(newUser) {
    if (user !== null)
      throw new AuthError('User already connected');
    return api.auth.register(newUser);
  }


  useEffect(() => {
    if (!api.auth.session())
      return;
    const promise = api.auth.refresh();
    setRequest({ type: 'refresh', promise });
  }, []);

  // Promises handling
  useEffect(() => {
    if (!request)
      return;

    let isSubscribed = true;
    const { promise, type } = request;

    promise.then(data => {
      if (!isSubscribed)
        return;

      if (['login', 'refresh'].includes(type))
        setUser(data.user) || setToken(jwtDecode(data.accessToken));

      else if ('logout' == type)
        return setUser(null) || setToken(null);

      setRequest(null);
    });

    return () => isSubscribed = false;
  }, [request]);

  // Refresh loop
  useEffect(() => {
    if (!token)
      return;

    let expiration = new Date(token.exp * 1000);
    let now = new Date();
    let timeout = expiration - now;

    console.log(`Refresh in ${timeout}...`);
    setTimeout(() => {
      let promise = api.auth.refresh().catch(
        () => { setUser(null); setToken(null); }
      );
      setRequest({ type: 'refresh', promise });
    }, timeout);
  }, [token]);


  useDebugValue(user ? 'Connected' : 'Anonymous');


  return (
    <AuthContext.Provider value={{ login, logout, register, user, token }}>
      {children}
    </AuthContext.Provider>
  );

}

export const useAuth = () => useContext(AuthContext);
