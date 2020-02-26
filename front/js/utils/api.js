
const root = '/api/v1';
const endpoints = Object.freeze(['login', 'logout', 'register']);

function api(endpoint, { body, ...providedConfig }) {
  
  if (!endpoints.contains(endpoint))
    throw new Error(`Unknown endpoint ${endpoint}`);

  const target = endpoints[endpoint];
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
    .fetch(`${root}/${target}`, config)
    .then(response => response.json());

}

function login(email, password) {
  return api('login', {
    body: { email, password }
  }).then(({ user, token }) => {
    window.localStorage.setItem('__auth_token__', token);
    return user;
  });
}

function logout() {
  return api('logout')
    .then(_ => window.localStorage.removeItem('__auth_token__'));
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