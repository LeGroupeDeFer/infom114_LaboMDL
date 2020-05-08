import React, {
  createContext,
  useContext,
  useEffect,
  useState,
  useDebugValue
} from 'react';
import api from '../lib/api';
import jwtDecode from 'jwt-decode';
import { printerr } from 'unanimity/lib';
import { useEffectQueue } from 'unanimity/hooks';
import {usePositiveEffect} from "../hooks";

class AuthError extends Error { }
const AuthContext = createContext(null);

let i = 0;
export function AuthProvider({ children }) {
    /* Internal state */
  const pushEffect = useEffectQueue();
  const [state, setState] = useState({
    user: null,
    token: null,

    login(email, password) {
      if (this.user !== null)
        throw new AuthError('User already connected');
      const promise = api.auth.login(email, password);
      pushEffect([
        promise,
        data => setState(state => ({
            ...state,
            user: data.user,
            token: jwtDecode(data.accessToken)
        })) || data,
        printerr // TODO
      ]);
      return promise;
    },

    logout() {
      const promise = api.auth.logout();
      pushEffect([
        promise,
        setState(state => ({ ...state, user: null, token: null })),
        printerr
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
    setState(state => ({ ...state, user: null, token: null })) // TODO
  ]), []);

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
      () => setState(state => ({ ...state, user: null, token: null }))
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
