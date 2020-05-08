import React, {
  createContext,
  useContext,
  useEffect,
  useState,
  useDebugValue
} from 'react';
import api from '../lib/api';
import jwtDecode from 'jwt-decode';
import { useEffectQueue, usePositiveEffect } from 'unanimity/hooks';

class AuthError extends Error { }
const AuthContext = createContext(null);

export function AuthProvider({ children }) {
  /* Internal state */
  const pushEffect = useEffectQueue();
  const [state, setState] = useState({
    user: null,
    token: null,
    error: null,

    login(email, password) {
      if (state.user !== null)
        throw new AuthError('User already connected');
      const promise = api.auth.login(email, password);
      pushEffect([
        promise,
        data => setState(state => ({
            ...state,
            user: data.user,
            token: jwtDecode(data.accessToken)
        })) || data,
        error => setState(state => ({ ...state, error }))
      ]);
      return promise;
    },

    logout() {
      const promise = api.auth.logout();
      pushEffect([
        promise,
        setState(state => ({ ...state, user: null, token: null })),
        error => setState(state => ({ ...state, error }))
      ]);
      return promise;
    },

    register(newUser) {
      if (this.user !== null)
        throw new AuthError('User already connected');
      return api.auth.register(newUser);
    }

  })

  useEffect(() => api.auth.session() && pushEffect([
    api.auth.refresh(),
    data => setState(state => ({
      ...state,
      user: data.user,
      token: jwtDecode(data.accessToken)
    })) || data,
    error => setState(state => ({ ...state, error, user: null, token: null }))
  ]) || undefined, []);

  // Refresh loop
  usePositiveEffect(() => {
    const expiration = new Date(state.token.exp * 1000);
    const now = new Date();

    setTimeout(() => pushEffect([
      api.auth.refresh(),
      data => setState(state => ({
        ...state,
        user: data.user,
        token: jwtDecode(data.accessToken)
      })),
      error => setState(state => ({ ...state, error, user: null, token: null }))
    ]), expiration - now);

  }, [state.token]);

  useDebugValue(state.user ? 'Connected' : 'Anonymous');

  return (
    <AuthContext.Provider value={state}>
      {children}
    </AuthContext.Provider>
  );

}

export const useAuth = () => useContext(AuthContext);
