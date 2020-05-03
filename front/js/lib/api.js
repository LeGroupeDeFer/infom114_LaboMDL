import { snake, camel } from './index';
import jwtDecode from 'jwt-decode';

/* istanbul ignore next */
const root = '/api/v1';
/* istanbul ignore next */
const store = window.localStorage;
/* istanbul ignore next */
let currentAccessToken;

/**
 * @memberof api
 *
 * @typedef { Object } User
 * @property { string } firstname
 * @property { string } lastname
 * @property { string } street
 * @property { number } number
 * @property { string } box
 * @property { string } city
 * @property { string } country
 * @property { string } phone
 */

/**
 * @memberof api
 *
 * @typedef { Object } Response
 * @property { boolean } success Request success status.
 * @property { string }  message A short description explaining the success status.
 * @property { int }     code [HTTP status code]{@link https://developer.mozilla.org/en-US/docs/Web/HTTP/Status}.
 */

/**
 * Fetch asynchronously the given api resource with the provided config.
 *
 * @namespace api
 *
 * @param { string } endpoint The api endpoint requested. **Do not** include `/api(/v[0-9])?` in it as it is already included.
 * @param { object } config The request configuration.
 * @param { body }   [config.body=null] The request payload, the request defaults to a `GET` method when this argument is null, to `POST` otherwise.
 * @param { ...any } [config.others=null] [Fetch parameters]{@link https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API} to override automatic parameters.
 */
function api(endpoint, { body, ...providedConfig } = {}) {
  const headers = { 'content-type': 'application/json' };

  if (currentAccessToken)
    headers['Authorization'] = `Bearer ${currentAccessToken}`;

  const config = {
    method: body ? 'POST' : 'GET',
    ...providedConfig,
    headers: {
      ...headers,
      ...providedConfig.headers,
    },
  };

  if (body) config.body = JSON.stringify(snake(body));

  return window
    .fetch(`${root}${endpoint}`, config)
    .then((response) =>
      Promise.all([
        new Promise((resolve, _) => resolve(response.status)),
        response.json(),
      ])
    )
    .then(([status, data]) => {
      if (status < 200 || status >= 300) throw { ...data, code: status };
      return camel(data);
    });
}

/* --------------------------------- Auth --------------------------------- */

function auth(endpoint, config) {
  return api(`/auth${endpoint}`, config);
}

Object.assign(auth, {
  clear() {
    currentAccessToken = undefined;
    store.removeItem('__refresh_token__');
  },

  /**
   * Attempts to login the user `email` with the given `password`. Succeeds with a [User]{@link api.User}, fails with a [Response]{@link api.Response}.
   *
   * @param {string} email
   * @param {string} password
   *
   * @returns {Promise<api.User|api.Response>}
   */
  login(email, password) {
    return auth('/login', {
      body: { email, password },
    }).then(({ user, accessToken, refreshToken }) => {
      currentAccessToken = accessToken;
      store.setItem('__refresh_token__', `${email}:${refreshToken}`);
      return { accessToken, user };
    });
  },

  /**
   * Attempts to logout the currently connected user.
   *
   * @returns {Promise<api.Response>}
   */
  logout() {
    const token = store.getItem('__refresh_token__');
    const [email, refreshToken] = token.split(':');

    if (refreshToken === null)
      return Promise.reject({ code: 0, reason: 'Not connected' });

    return auth('/logout', { body: { email, refreshToken } }).then(
      (data) => auth.clear() || data
    );
  },

  refresh() {
    const token = store.getItem('__refresh_token__');
    const [email, refreshToken] = token.split(':');

    return auth('/refresh', { body: { email, refreshToken } })
      .then(({ accessToken, refreshToken, user }) => {
        currentAccessToken = accessToken;
        store.setItem('__refresh_token__', `${email}:${refreshToken}`);
        return { accessToken, user };
      })
      .catch(({ code, reason }) => {
        if (code == 403)
          // Token expired
          auth.clear();
        return Promise.reject({ code, reason });
      });
  },

  /**
   * Creates an account with the given information.
   *
   * @param {User} user
   * @returns {Promise<api.Response>}
   */
  register(user) {
    if (!user.terms)
      throw new Error('Must accept terms and conditions before to register');
    return auth('/register', { body: user });
  },

  activate(id, token) {
    return auth('/activate', { body: { id: Number(id), token } });
  },

  restore(email) {
    return auth('/restore', { body: { email } });
  },

  recover(id, password, token) {
    return auth('/recover', { body: { id: Number(id), password, token } });
  },

  session() {
    return store.getItem('__refresh_token__') !== null;
  },
});

/* --------------------------------- Posts -------------------------------- */

function posts() {
  return api('/posts');
}

Object.assign(posts, {
  of(id) { return api(`/post/${id}`); },
  vote(id, vote) { return api(`/post/${id}/vote`, { body: { vote: vote } }); },
  add(post) { return api(`/post`, { body: post }); }
});

/* --------------------------------- Tags --------------------------------- */

function tags(id) {
  return api('/tags/');
}

Object.assign(api, { auth, posts, tags });


export default api;
