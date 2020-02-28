
const root = '/api/v1';
const endpoints = Object.freeze(['login', 'logout', 'register']);

/**
 * @typedef { Object } User
 * @property { string } firstname
 * @property { string } lastname
 * @property { string } street
 * @property { number } number
 * @property { string } city
 * @property { string } country
 * @property { string } phone
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
        throw new Error(data.message);
      return data;
    });

}

/**
 * Attempts to login the user `email` with the given `password`. If the login
 * is successful, returns the logged user.
 * 
 * @param {string} email 
 * @param {string} password 
 * 
 * @returns {User}
 */
function login(email, password) {
  return api('login', {
    body: { email, password }
  }).then(({ user, token }) => {
    window.localStorage.setItem('__auth_token__', token || 'test_token');
    return user;
  });
}

function logout() {
  return api('logout', {})
    .then(r => window.localStorage.removeItem('__auth_token__') || r);
}

/**
 * Creates an account with the given information.
 * 
 * @param {Object} user
 * @param {string} user.email
 * @param {string} user.password
 * @param {string} user.firstname
 * @param {string} user.number
 * @param {string} user.city
 * @param {number} user.zipcode
 * @param {string} user.phone
 */
function register(user) {
  return api('register', { body: user });
}


api.endpoints = endpoints;
api.login = login;
api.logout = logout;
api.register = register;


export default api;
