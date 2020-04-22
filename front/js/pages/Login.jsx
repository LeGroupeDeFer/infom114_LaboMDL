import { faArrowLeft, faArrowRight } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import React, { useEffect, useState } from 'react';
import Col from 'react-bootstrap/Col';
import Container from 'react-bootstrap/Container';
import Form from 'react-bootstrap/Form';
import Row from 'react-bootstrap/Row';
import { Link, useHistory } from 'react-router-dom';
import { useAuth } from '../context/authContext';
import AutoForm from '../components/AutoForm';
import Flexbox from '../components/Flexbox';
import { isUnamurEmail, isValidPassword } from '../lib/validators';


function Header(props) {

  return (
    <Flexbox justify="center" align="end" className='mb-3 form-header'>
      <h4 className="mb-1 mx-2">
        <Link to='/' className="text-secondary">
          <Icon icon={faArrowLeft} className="mr-2" />
          HOME
        </Link>
      </h4>
      <h1 className="mb-0 mx-2">
        SIGN IN
      </h1>
      <h4 className="mb-1 mx-2">
        <Link to='/register' className="text-secondary">
          SIGN UP
          <Icon icon={faArrowRight} className="ml-2" />
        </Link>
      </h4>
    </Flexbox>
  );

}


function LoginForm() {

  return (
    <>
      <Form.Group className="form-group-material">
        <AutoForm.Control
          id="email"
          name="email"
          type="email"
          validator={isUnamurEmail}
        />
        <Form.Label><small><b>EMAIL</b></small></Form.Label>
        <div className="underline" />
        <div className="highlight" />
      </Form.Group>

      <Form.Group className='form-group-material'>
        <AutoForm.Control
          id="password"
          name="password"
          type="password"
          eraseOnFailure={true}
          validator={isValidPassword}
        />
        <Form.Label><small><b>PASSWORD</b></small></Form.Label>
        <span className="underline" />
        <div className="highlight" />
      </Form.Group>

      <AutoForm.Submit
        variant="secondary"
        className="d-block px-5 my-2 mx-auto"
      >Login</AutoForm.Submit>
    </>
  );

}


const Login = Unauthenticated(() => {

  const { login } = useAuth();
  const history = useHistory();

  const handleSubmit = data =>
    login(data.email, data.password).then(_ => history.push('/'));

  return (
    <Container className="login-form">
      <Row>
        <Col md={{ span: 8, offset: 2 }} lg={{ span: 6, offset: 3 }}>

          <AutoForm onSubmit={handleSubmit} autoComplete="off">
            <Header />
            <hr />

            <LoginForm />
            <hr />

            <Link to="/restore" className="d-block text-center text-light">
              Forgot your password?
            </Link>
          </AutoForm>

        </Col>
      </Row>
    </Container>
  );
});


export default Login;
