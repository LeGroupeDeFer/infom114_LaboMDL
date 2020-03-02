import React from 'react';
import { useHistory } from 'react-router-dom';
import { useAuth } from '../context/authContext';


function Register() {

  const history = useHistory();
  const { user, register } = useAuth();

  if (user) // TODO - flash already connected
    history.replace('/');

  return (
    <h1>Register</h1>
  );

}

export default Register;
