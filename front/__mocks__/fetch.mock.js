
const fetch = jest.fn();
let connectedUser = null;


const Response = (data, status = 200) => Promise.resolve({
  ok: status >= 200 && status < 400,
  status,
  json: () => Promise.resolve({
    ...data,
    success: status >= 200 && status < 400
  }),
});

const fakeUser = {
  email: 'jdoe@student.unamur.be',
  password: 'secret',
  firstname: 'John',
  lastname: 'Doe',
  street: 'Evergreen Terass',
  number: 742,
  city: 'Springfield',
  zipcode: 1020,
  country: 'USA',
  phone: '636-555-3226'
};

const fakeToken = 'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJVbmFuaW1pdHkiLCJpYXQiOjE1ODQ3MDk2MTAsImV4cCI6MTU4NTkxOTIxMCwic3ViIjoxMCwiY2FwIjpbXX0.HXBQgrw5wKXTfpkMCuNaO-OkfR89oxbkg8Z0VUNHuCU';

function apiLogin(data, method = 'GET') {
  const { email, password } = data;

  if (method === 'GET')
    return Response({});

  if (!email || !password)
    return Response({ success: false, message: 'Missing property' }, 422);

  if (email === fakeUser.email && password === fakeUser.password) {
    connectedUser = fakeUser;
    return Response({ token: fakeToken, user: fakeUser });
  }

  if (email === 'unverified@unamur.be')
    return Response({ success: false, message: 'Unverified email' }, 403);

  return Response({ success: false, message: 'Wrong email/password association' }, 401);

}

function apiRegister(data, method = 'GET') {

  if (method === 'GET')
    return Response({});

  const { email, password, firstname, lastname, street, number, box, city,
    zipcode, country, phone, terms } = data;

  if (!email || !password || !firstname || !lastname || !terms)
    return Response({ success: false, message: 'Missing property' }, 422);

  if (email === fakeUser.email)
    return Response({ success: false, message: 'Email already in use' }, 409);

  return Response({ success: true, message: 'registered' });
}

function apiLogout(data, method = 'GET') {
  if (method === 'POST' && connectedUser === null)
    return Response({ success: false, message: 'Not connected' }, 403);

  connectedUser = null;
  return Response({});
}


fetch.mockImplementation((endpoint, { body, ...config }) => {

  const method = config.method || 'GET';
  let data = body ? JSON.parse(body) : {};

  switch (endpoint) {
    case '/api/auth/login':
    case '/api/v1/auth/login':
      return apiLogin(data, method);

    case '/api/auth/register':
    case '/api/v1/auth/register':
      return apiRegister(data, method);

    case '/api/auth/logout':
    case '/api/v1/auth/logout':
      return apiLogout(data, method);

    default:
      return Response({ success: false, message: 'Unknown page' }, 404);
  }

});

fetch.removeUser = () => { connectedUser = null; };
fetch.fakes = {
  user: fakeUser,
  token: fakeToken
};

global.fetch = window.fetch = fetch;
global.Response = window.Response = Response;