import { snake, camel, empty, trace, identity, clean } from './index';
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

const encode = encodeURIComponent;
function query(target, search = {}) {
  const snakeSearch = snake(search);
  const params = Object.keys(snakeSearch)
    .map((key) =>
      snakeSearch[key] instanceof Array
        ? `${key}=${snakeSearch[key].map(encode).join(':')}`
        : `${key}=${encode(snakeSearch[key])}`
    )
    .filter(identity);

  return empty(params) ? target : `${target}?${params.join('&')}`;
}

Object.assign(query, { encode });

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

  const method = providedConfig.method || (body ? 'POST' : 'GET');
  const config = {
    method,
    ...providedConfig,
    headers: {
      ...headers,
      ...providedConfig.headers,
    },
  };

  let target = `${root}${endpoint}`;
  if (body)
    if (method === 'GET') target = query(target, clean(body));
    else config.body = JSON.stringify(snake(clean(body)));

  return window
    .fetch(target, config)
    .then((response) =>
      Promise.all([
        new Promise((resolve, _) => resolve(response.status)),
        response.headers.get('Content-Type').includes('application/json')
          ? response.json()
          : response.text(),
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
  of(id) {
    return api(`/post/${id}`);
  },
  vote(id, vote) {
    return api(`/post/${id}/vote`, { body: { vote } });
  },
  add(post) {
    return api('/post', { body: post });
  },
  delete(id) {
    return api(`/post/${id}`, { method: 'DELETE' });
  },
  hide(id) {
    return api(`/post/${id}/hide`, { method: 'POST' });
  },
  lock(id) {
    return api(`/post/${id}/lock`, { method: 'POST' });
  },
  where(query) {
    return api('/posts', { method: 'GET', body: query });
  },
  flag(id, reason, cancel) {
    if (cancel) return api(`/post/${id}/report`, { method: 'POST' });
    return api(`/post/${id}/report`, { method: 'POST', body: { reason } });
  },
  watch(id) {
    return api(`/post/${id}/watch`, { method: 'POST' });
  },
  pollData(id) {
    return api(`/post/${id}/poll`);
  },
  pollVote(postId, answerId) {
    return api(`/post/${postId}/poll`, { method: 'POST', body: { answerId } });
  },
});

/* --------------------------------- Tags --------------------------------- */

function tags(id) {
  return api('/tags');
}

Object.assign(api, { query, auth, posts, tags });

/* TODO Move all tag, role & users related fns in their respective namespace */

function addTag(label) {
  return api(`/tag/${label}`, { method: 'POST' });
}

function removeTag(label) {
  return api(`/tag/${label}`, { method: 'DELETE' });
}

function editTag(oldLabel, newLabel) {
  return api(`/tag/${oldLabel}`, {
    method: 'PUT',
    body: { label: String(newLabel) },
  });
}

function roles() {
  return api('/roles');
}

function addRole(name, color, capability) {
  return api('/role', {
    method: 'POST',
    body: {
      name: String(name),
      color: String(color),
      capabilities: [{ name: String(capability) }],
    },
  });
}

function editRole(id, name, color, capabilities) {
  return api(`/role/${id}`, {
    method: 'PUT',
    body: {
      name: String(name),
      color: String(color),
      capabilities: capabilities,
    },
  });
}

function deleteRole(id) {
  return api(`/role/${id}`, { method: 'DELETE' });
}

function capabilities() {
  return api('/capabilities');
}

function users() {
  return api('/users');
}

function addRoleToUser(userID, roleID) {
  return api('/user/role', {
    method: 'POST',
    body: { user_id: userID, role_id: parseInt(roleID) },
  });
}

function removeRoleFromUser(userID, roleID) {
  return api('/user/role', {
    method: 'DELETE',
    body: { user_id: userID, role_id: parseInt(roleID) },
  });
}

function userStat() {
  return api('/report/users');
}

function tagStat() {
  return api('/report/tags');
}

function postStat() {
  return api('/report/activity');
}

function postFlagged() {
  return api('/report/post_reported');
}

function userPosts(id) {
  return api(`/user/${id}/posts`)
}

api.tags = tags;
api.tags.add = addTag;
api.tags.remove = removeTag;
api.tags.edit = editTag;
api.tag = tags;
api.roles = roles;
api.roles.add = addRole;
api.roles.edit = editRole;
api.roles.delete = deleteRole;
api.capabilities = capabilities;
api.users = users;
api.users.addRole = addRoleToUser;
api.users.removeRole = removeRoleFromUser;
api.users.posts = userPosts;
api.users.report = userStat;
api.tags.report = tagStat;
api.posts.report = postStat;

api.posts.flagged = postFlagged;

export default api;
