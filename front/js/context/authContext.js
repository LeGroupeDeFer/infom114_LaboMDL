import React, {
  createContext,
  useContext,
  useEffect,
  useState,
  useLayoutEffect,
  useDebugValue
} from 'react';
import api from 'unanimity/utils/api';


class AuthError extends Error { }
const AuthContext = createContext();
const store = window.localStorage;


export function AuthProvider(props) {

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
    if (user !== null)
      throw new AuthError('No user connected');
    return api.logout().then(_ => setUser(null));
  }

  function register(user) {
    if (user !== null)
      throw new AuthError('User already connected');
    return api.register(user);
  }

  return (
    <AuthContext.Provider value={{ login, logout, register, user }}>
      {props.children}
    </AuthContext.Provider>
  );

}

export const useAuth = () => useContext(AuthContext);
