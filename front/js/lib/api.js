import { snake, camel } from './index';
import jwtDecode from 'jwt-decode';

/* istanbul ignore next */
const root = '/api/v1';
/* istanbul ignore next */
const store = window.localStorage;

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

  const token = store.getItem('__auth_token__');
  const headers = { 'content-type': 'application/json' };

  if (token)
    headers['Authorization'] = `Bearer ${token}`;

  const config = {
    method: body ? 'POST' : 'GET',
    ...providedConfig,
    headers: {
      ...headers,
      ...providedConfig.headers
    }
  };

  if (body)
    config.body = JSON.stringify(snake(body));

  return window
    .fetch(`${root}${endpoint}`, config)
    .then(response => Promise.all(
      [new Promise(resolve => resolve(response.status)), response.json()]
    ))
    .then(([status, data]) => {
      if (status < 200 || status >= 300)
        throw { ...data, code: status };
      return camel(data);
    });

}

function auth(endpoint, config) {
  return api(`/auth${endpoint}`, config);
}

/**
 * Attempts to login the user `email` with the given `password`. Succeeds with a [User]{@link api.User}, fails with a [Response]{@link api.Response}.
 * @memberof api
 *
 * @param {string} email
 * @param {string} password
 *
 * @returns {Promise<api.User|api.Response>}
 */
function login(email, password) {
  return api.auth('/login', {
    body: { email, password }
  }).then(({ user, accessToken, refreshToken }) => {

    store.setItem('__refresh_token__', `${email}:${refreshToken}`);
    return { accessToken, user };
  });

}

/**
 * Attempts to logout the currently connected user.
 * @memberof api
 *
 * @returns {Promise<api.Response>}
 */
function logout() {
  const token = store.getItem('__refresh_token__');
  const [email, refreshToken] = token.split(':');

  if (refreshToken === null)
    return Promise.reject({ reason: 'Not connected' });

  return api.auth('/logout', { body: { email, refreshToken }})
    .then(data => {
      store.removeItem('__refresh_token__');
      return data;
    });

}

function refresh() {
  const token = store.getItem('__refresh_token__');
  const [email, refreshToken] = token.split(':');

  return api.auth('/refresh', { body: { email, refreshToken } })
    .then(({ refreshToken, ...others }) => {
      store.setItem('__refresh_token__', `${email}:${refreshToken}`);
      return others;
    })
    .catch(({ code, reason }) => {
      if (code == 403) // Token expired
        store.clear();
      return Promise.reject({ code, reason });
    });
}

/**
 * Creates an account with the given information.
 * @memberof api
 *
 * @param {User} user
 * @returns {Promise<api.Response>}
 */
function register(user) {
  if (!user.terms)
    throw new Error('Must accept terms and conditions before to register');
  return api.auth('/register', { body: user });
}

function activate(id, token) {
  return api('/auth/activate', { body: { id: Number(id), token } });
}

function recover(id, token) {
  return api('/auth/recover', { body: { id: Number(id), token } });
}

function session() {
  return store.getItem('__refresh_token__') !== null;
}

api.auth = auth;
api.auth.login = login;
api.auth.logout = logout;
api.auth.register = register;
api.auth.activate = activate;
api.auth.refresh = refresh;
api.auth.session = session;

export default api;
