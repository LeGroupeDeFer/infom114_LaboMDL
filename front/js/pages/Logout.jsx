import React from 'react';
import { useAuth } from '../context';
import { useEffect } from 'react';
import { useHistory } from 'react-router-dom';
import { Authenticated } from '../components';

// Not a real component, just logout the user before to redirect to "/".
const Logout = Authenticated(() => {
  const { logout } = useAuth();
  const history = useHistory();

  useEffect(() => {
    logout().then(_ => history.push('/'));
  }, []);

  return <></>;
});


export default Logout;
