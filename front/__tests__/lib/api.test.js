import regeneratorRuntime from 'regenerator-runtime';
import api from '../../js/lib/api';
import '../../__mocks__/fetch.mock';
import '../../__mocks__/storage.mock';

describe('api', () => {

  beforeAll(() => { localStorage.setItem('__auth_token__', 'secret'); });
  afterAll(() => { localStorage.removeItem('__auth_token__'); });

  it('should redirect to the api', async () => {
    await api('/auth/login');

    expect(fetch).toHaveBeenCalledWith('/api/v1/auth/login', expect.anything());
  });

  it('should reject invalid endpoints', async () => {
    try {
      await api('/iDefinitelyExist');
    } catch (e) {
      expect(e.code).toBe(404);
    }
  });

  it('should attach a sane default configuration', async () => {
    // We don't want per-se to get to /login but it's an available endpoint
    await api('/auth/login');
    expect(fetch).toHaveBeenCalledWith(expect.anything(), {
      method: 'GET',
      headers: {
        'Authorization': 'Bearer secret',
        'content-type': 'application/json'
      }
    });
  });

  it('should convey the payload', async () => {
    await api('/auth/login', {
      body: {
        email: 'jdoe@student.unamur.be',
        password: 'secret',
      }
    });
    expect(fetch).toHaveBeenCalledWith(expect.anything(), {
      method: 'POST',
      body: JSON.stringify({
        email: 'jdoe@student.unamur.be',
        password: 'secret',
      }),
      headers: {
        'Authorization': 'Bearer secret',
        'content-type': 'application/json'
      }
    });
  });

  it('should fail on 4xx+ error codes', async () => {
    let error = false;
    try {
      await api('/login', { body: { thing: 'thing' } });
    } catch (e) { error = true; }

    expect(error).toBeTruthy();
  });

});

describe('api.login', () => {

  beforeEach(localStorage.clear);
  afterEach(fetch.removeUser);

  it('should log in the user', async () => {
    const { user, token } = await api.login('jdoe@student.unamur.be', 'secret');
    expect(user).toBeTruthy();
  });

  it('should setup a token', async () => {
    await api.login('jdoe@student.unamur.be', 'secret');
    const token = localStorage.getItem('__auth_token__');
    expect(token).toBeTruthy();
  });

});

describe('api.logout', () => {

  beforeEach(localStorage.clear);
  afterEach(fetch.removeUser);

  it('should not do anything when the user is logged in', async () => {
    await api.logout();
  });

  it('should logout the user', async () => {
    await api.login('jdoe@student.unamur.be', 'secret');
    const loggedIntoken = localStorage.getItem('__auth_token__')

    await api.logout();
    const loggedOuttoken = localStorage.getItem('__auth_token__');

    expect(loggedOuttoken).not.toBe(loggedIntoken);
    expect(loggedOuttoken).toBe(null);
  });

});

describe('api.register', () => {

  it('should register the user', async () => {
    // We simply test that there is no exception thrown
    await api.register({
      email: 'joe@student.unamur.be',
      password: 'secret',
      firstname: 'John',
      lastname: 'Doe',
      street: 'Evergreen Terass',
      number: 742,
      city: 'Springfield',
      zipcode: 1020,
      country: 'USA',
      phone: '636-555-3226',
      terms: true
    });
  });

  it('should reject incorrect registration', async () => {
    let error = false;
    try {
      await api.register({
        email: 'joe2@student.unamur.be',
        password: 'secret',
        firstname: 'John',
        lastname: 'Doe',
        street: 'Evergreen Terass',
        number: 742,
        city: 'Springfield',
        zipcode: 1020,
        country: 'USA',
        phone: '636-555-3226',
        terms: false
      })
    } catch (e) { error = true; }

    expect(error).toBe(true);
  });

});