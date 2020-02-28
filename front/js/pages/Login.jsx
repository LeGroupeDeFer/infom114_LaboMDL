import React, { useEffect, useState } from 'react';
import useInput from '../hooks/useInput';
import Form from 'react-bootstrap/Form';
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


function Login(props) {

  const { value: email, bind: bindEmail, reset: resetEmail } = useInput("");
  const { value: password, bind: bindPassword, reset: resetPassword } = useInput("");
  const [valid, setValid] = useState(true);
  const { login, user } = useAuth();
  const history = useHistory();

  useEffect(() => {
    if (user)
      history.replace('/');
  }, [user]);

  useEffect(() => {
    if (valid)
      return;
    setValid(false);
    setTimeout(() => {
      setValid(true);
    }, 300);
  }, [valid]);

  const handleSubmit = e => {
    e.preventDefault();
    login(email, password)
      .then(_ => history.push('/'))
      .catch(e => {
        console.log(e);
        setValid(false);
        resetEmail();
        resetPassword();
      });
  }

  return (
    <Container className="login-form">
      <Row>
        <Col lg={{ span: 6, offset: 3 }}>

          <Flexbox justify="center" align="end" className='mb-3'>
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
          <hr />

          <Form onSubmit={handleSubmit} validated>
            <Form.Group controlId="loginEmail">
              <Form.Label><small><b>EMAIL</b></small></Form.Label>
              <Form.Control
                required
                type="email"
                placeholder="you@unamur.be"
                className="form-control"
                {...bindEmail}
              />
            </Form.Group>
            <Form.Group controlId="loginPassword">
              <Form.Label><small><b>PASSWORD</b></small></Form.Label>
              <Form.Control
                required
                type="password"
                {...bindPassword}
              />
            </Form.Group>
            <Button
              variant={valid ? "secondary" : "danger"}
              type="submit"
              className="d-block px-5 mb-4 mx-auto"
            >Login</Button>
          </Form>
          <hr />
          <Link to="/recovery" className="d-block text-center text-light">
            Forgot your password?
          </Link>
        </Col>
      </Row>
    </Container>
  );
}

export default Login;
