import React from 'react';
import { useAuth } from '../context/authContext';
import { useEffect } from 'react';
import { useHistory } from 'react-router-dom';

// Not a real component, just logout the user before to redirect to "/".
function Logout(props) {
  const { logout, user } = useAuth();
  const history = useHistory();

  useEffect(() => {
    if (user)
      logout().then(_ => history.replace('/'));
    else
      history.replace('/');
  }, []);

  return <></>;
}


export default Logout;
