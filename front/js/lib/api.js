
const root = '/api/v1';
const endpoints = Object.freeze(['login', 'logout', 'register']);

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
function api(endpoint, { body, ...providedConfig }) {

  if (!endpoints.includes(endpoint))
    throw new Error(`Unknown endpoint ${endpoint}`);

  const token = window.localStorage.getItem('__auth_token__');
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
    config.body = JSON.stringify(body);

  return window
    .fetch(`${root}/${endpoint}`, config)
    .then(response => Promise.all(
      [new Promise(resolve => resolve(response.status)), response.json()]
    ))
    .then(([status, data]) => {
      if (status < 200 || status >= 300 || !data.success)
        throw { ...data, code: status };
      return data;
    });

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
  return api('login', {
    body: { email, password }
  }).then(({ user, token }) => {
    window.localStorage.setItem('__auth_token__', token || 'test_token');
    return user;
  });
}
/**
 * Attempts to logout the currently connected user. **Does not check if the user is connected** prior to the disconnection attempt.
 * @memberof api
 * 
 * @returns {Promise<api.Response>}
 */
function logout() {
  return api('logout', {})
    .then(r => window.localStorage.removeItem('__auth_token__') || r);
}

/**
 * Creates an account with the given information.
 * @memberof api
 * 
 * @param {User} user
 * @returns {Promise<api.Response>}
 */
function register(user) {
  return api('register', { body: user });
}


api.endpoints = endpoints;
api.login = login;
api.logout = logout;
api.register = register;


export default api;
