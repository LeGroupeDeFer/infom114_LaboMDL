import React, { useEffect, useState } from 'react';
import useInput from '../hooks/useInput';
import Form from 'react-bootstrap/Form';
import AutoForm from '../components/AutoForm';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import Container from 'react-bootstrap/Container';
import Button from 'react-bootstrap/Button';
import api from '../utils/api';
import Flexbox from '../components/Flexbox';
import { useHistory, Link } from 'react-router-dom';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { faArrowLeft, faArrowRight } from '@fortawesome/free-solid-svg-icons';
import { useAuth } from 'unanimity/context/authContext';
import { isUnamurEmail, isValidPassword } from '../utils/validators';


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
      <Form.Group controlId="loginEmail">
        <Form.Label><small><b>EMAIL</b></small></Form.Label>
        <AutoForm.Control
          required
          name="email"
          type="email"
          placeholder="you@unamur.be"
          validator={isUnamurEmail}
        />
      </Form.Group>

      <Form.Group controlId="loginPassword">
        <Form.Label><small><b>PASSWORD</b></small></Form.Label>
        <AutoForm.Control
          required
          name="password"
          type="password"
          eraseOnFailure={true}
          validator={isValidPassword}
        />
      </Form.Group>

      <AutoForm.Submit
        variant="secondary"
        className="d-block px-5 my-2 mx-auto"
      >Login</AutoForm.Submit>
    </>
  );

}


function Login(props) {

  const { login, user } = useAuth();
  const history = useHistory();
  useEffect(() => user ? history.replace('/') : undefined, [user]);
  const [error, setError] = useState(false);

  const handleSubmit = data => {
    const { email, password } = data;

    login(email, password)
      .then(_ => history.push('/'))
      .catch(error => setError(error));
  }

  return (
    <Container className="login-form">
      <Row>
        <Col lg={{ span: 6, offset: 3 }}>

          <AutoForm error={error} onSubmit={handleSubmit}>
            <Header />
            <hr />

            <LoginForm />
            <hr />

            <Link to="/recovery" className="d-block text-center text-light">
              Forgot your password?
            </Link>
          </AutoForm>

        </Col>
      </Row>
    </Container>
  );
}

export default Login;
