import regeneratorRuntime from 'regenerator-runtime';
import api from '../../js/lib/api';

const fakeResponse = (data, status=200) => Promise.resolve({
  ok: status >= 200 && status < 400,
  status,
  json: () => Promise.resolve({
    ...data,
    success: status >= 200 && status < 400
  }),
});

describe('api', () => {

  const store = { '__auth_token__': 'secret' };

  beforeAll(() => {
    global.fetch = jest.fn();
    Object.defineProperty(window, 'localStorage', {
      writable: true,
      value: {
        getItem: key => key in store ? store[key] : null,
        setItem: (key, value) => {
          store[key] = value;
        },
      }
    });
  });

  it('should redirect to the api', async () => {
    fetch.mockImplementation(() => fakeResponse({}));
    await api('/login');

    expect(fetch).toHaveBeenCalledWith('/api/v1/login',  expect.anything());
  });

  it('should reject invalid endpoints', async () => {
    fetch.mockImplementation(() => fakeResponse({}));

    try {
      await api('/iDefinitelyExist');
    } catch(e) {
      expect(e).toBeInstanceOf(Error);
    }
  });

  it('should attach a sane default configuration', async () => {
    fetch.mockImplementation(() => fakeResponse({}));

    // We don't want per-se to get to /login but it's an available endpoint
    await api('/login');
    expect(fetch).toHaveBeenCalledWith(expect.anything(), {
      method: 'GET',
      headers: {
        'Authorization': 'Bearer secret',
        'content-type': 'application/json'
      }
    });
  });

  it('should convey the payload', async () => {
    fetch.mockImplementation(() => fakeResponse({}));

    await api('/login', { body: { things: ['one thing', 'another', 'yet another'] } });
    expect(fetch).toHaveBeenCalledWith(expect.anything(), {
      method: 'POST',
      body: JSON.stringify({ things: ['one thing', 'another', 'yet another'] }),
      headers: {
        'Authorization': 'Bearer secret',
        'content-type': 'application/json'
      }
    });
  });

  it('should fail on 4xx+ error codes', async () => {
    fetch.mockImplementation(() => fakeResponse({}, 401));

    let error = false;
    try { await api('/login'); } catch (e) { error = true; }

    expect(error).toBeTruthy();
  });

});

describe.skip('api.login', () => {
  // TODO
});

describe.skip('api.logout', () => {
  // TODO
});

describe.skip('api.register', () => {
  // TODO
});