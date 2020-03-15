import regeneratorRuntime from 'regenerator-runtime';
import React, { useState } from 'react';
import '../../__mocks__/fetch.mock';
import '../../__mocks__/storage.mock';
import { AuthProvider, useAuth } from '../../js/context/authContext';
import { act, fireEvent, render, wait } from '@testing-library/react';



class ErrorBoundary extends React.Component {
  constructor(props) {
    super(props);
    this.state = { hasError: false };
  }

  static getDerivedStateFromError(error) {
    return { hasError: true };
  }

  componentDidCatch(error, errorInfo) {
  }

  render() {
    if (this.state.hasError) {
      return <h1 id="test-error">Something went wrong.</h1>;
    }

    return this.props.children;
  }

}


function DebugAuthProvider() {

  const { user } = useAuth();

  return (
    <>
      <div className='test-user'>{user ? user.toString() : ''}</div>
    </>
  );

}

function LogMeIn() {

  const [error, setError] = useState(null);
  if (error)
    throw error;

  const { login } = useAuth();
  // Convoluted callback to get the error to go from effect phase to rendering
  // phase and be caught by the error boundary
  const callback = () => {
    try { login('jdoe@student.unamur.be', 'secret'); } catch (e) { setError(e); }
  };

  return <button className='login' onClick={callback} />;

}

function LogMeOut() {

  const { logout } = useAuth();

  return <button className='logout' onClick={logout} />;

}

function RegisterMe() {

  const [error, setError] = useState(null);
  if (error)
    throw error;

  const { register } = useAuth();
  const callback = () => {
    try {
      register({
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
    } catch (e) { setError(e); }
  };

  return <button className='register' onClick={callback} />;

}


describe('authContext', () => {

  beforeEach(localStorage.clear);
  afterEach(fetch.removeUser);

  it('should render', () => {
    let error = false;
    try { render(<AuthProvider />); }
    catch (e) { error = true; }
    expect(error).toBe(false);
  });

  it('should not have a user', () => {
    const { container } = render(
      <AuthProvider>
        <DebugAuthProvider />
      </AuthProvider>
    );

    const user = container.querySelector('.test-user');
    expect(user.textContent).toBe('');
  });

  it('should connect the user', async () => {
    const { container } = render(
      <AuthProvider>
        <LogMeIn />
        <DebugAuthProvider />
      </AuthProvider>
    );

    const loginButton = container.querySelector('button');
    fireEvent.click(loginButton);
    const user = container.querySelector('.test-user');
    await wait(() => {
      expect(user.textContent).toBeTruthy();
      expect(localStorage.getItem('__auth_user__')).not.toBe(null);
    });
  });

  it('should load the user from memory', async () => {
    localStorage.setItem('__auth_user__', JSON.stringify({
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
    }));

    const { container } = render(
      <AuthProvider>
        <DebugAuthProvider />
      </AuthProvider>
    );

    const user = container.querySelector('.test-user');
    await wait(() => expect(user.textContent).toBeTruthy());
  });

  it('should not connect the user more than once', async () => {
    const spy = jest.spyOn(console, 'error').mockImplementation(() => { });

    const { container } = render(
      <ErrorBoundary>
        <AuthProvider>
          <LogMeIn />
        </AuthProvider>
      </ErrorBoundary>
    );

    const loginButton = container.querySelector('button');

    await wait(() => fireEvent.click(loginButton));
    await wait(() => fireEvent.click(loginButton));
    const error = container.querySelector('#test-error');
    await wait(() => expect(error).toBeTruthy());

    spy.mockRestore();
  });

  it('should disconnect the user', async () => {
    const { container } = render(
      <AuthProvider>
        <LogMeIn />
        <LogMeOut />
        <DebugAuthProvider />
      </AuthProvider>
    );

    const loginButton = container.querySelector('.login');
    fireEvent.click(loginButton);
    const user = container.querySelector('.test-user');
    await wait(() => expect(user.textContent).toBeTruthy());

    const logoutButton = container.querySelector('.logout');
    fireEvent.click(logoutButton);
    await wait(() => expect(user.textContent).toBeFalsy());
  });

  it('should register the user', async () => {
    const { container } = render(
      <AuthProvider>
        <RegisterMe />
        <DebugAuthProvider />
      </AuthProvider>
    );

    const button = container.querySelector('button');
    fireEvent.click(button);
  });

  it('should not register if the user is connected', async () => {
    const spy = jest.spyOn(console, 'error').mockImplementation(() => { });

    const { container } = render(
      <ErrorBoundary>
        <AuthProvider>
          <LogMeIn />
          <RegisterMe />
        </AuthProvider>
      </ErrorBoundary>
    );

    const loginButton = container.querySelector('.login');
    const registerButton = container.querySelector('.register');

    await wait(() => fireEvent.click(loginButton));
    await wait(() => fireEvent.click(registerButton));
    const error = container.querySelector('#test-error');
    await wait(() => expect(error).toBeTruthy());

    spy.mockRestore();
  });

});