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
    pending: false,
    timer: null,

    login(email, password) {
      if (state.user !== null)
        return;
      setState(s => ({ ...s, pending: true, error: null }));
      const promise = api.auth.login(email, password);
      pushEffect([
        promise,
        data => setState(s => ({
          ...s,
          pending: false,
          user: data.user,
          error: null,
          token: jwtDecode(data.accessToken)
        })) || data,
        error => setState(s => ({ ...s, pending: false, error }))
      ]);
      return promise;
    },

    logout() {
      const promise = api.auth.logout();
      pushEffect([
        promise,
        setState(s => ({ ...s, user: null, token: null, error: null })),
        error => setState(s => ({ ...s, error }))
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
    setState(s => ({ ...s, pending: true, error: null })) || api.auth.refresh(),
    data => setState(state => ({
      ...state,
      error: null,
      pending: false,
      user: data.user,
      token: jwtDecode(data.accessToken)
    })) || data,
    error => setState(state => ({ ...state, pending: false, error, user: null, token: null }))
  ]) || undefined, []);

  // Refresh loop
  useEffect(() => {
    if (!state.token) {
      if (state.timer)
        clearTimeout(state.timer) || setState(s => ({ ...s, timer: null }));
      return;
    }

    const expiration = new Date(state.token.exp * 1000);
    const now = new Date();

   const timer = setTimeout(() => pushEffect([
      api.auth.refresh(),
      data => setState(s => ({
        ...s,
        user: data.user,
        error: null,
        token: jwtDecode(data.accessToken)
      })),
      error => setState(s => ({ ...s, error, user: null, token: null }))
    ]), expiration - now);

    setState(s => ({ ...s, timer }));
  }, [state.token]);

  useDebugValue(state.user ? 'Connected' : 'Anonymous');

  return (
    <AuthContext.Provider value={state}>
      {children}
    </AuthContext.Provider>
  );

}

export const useAuth = () => useContext(AuthContext);
