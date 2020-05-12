import React from 'react';
import { Container, Row, Col, Form } from 'react-bootstrap';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { Link, useHistory } from 'react-router-dom';
import { useAuth } from '../context';
import { AutoForm, Flexbox, Unauthenticated } from '../components';
import { isUnamurEmail, isValidPassword } from '../lib/validators';
import { Simple as SimpleError } from 'unanimity/components';


function Header() {

  return (
    <Flexbox justify="center" align="end" className='mb-3 form-header'>
      <h5 className="mb-1 mx-2">
        <Link to='/' className="text-secondary">
          <Icon icon="arrow-left" className="mr-2" />
          ACCUEIL
        </Link>
      </h5>
      <h2 className="mb-0 mx-2">
        CONNEXION
      </h2>
      <h5 className="mb-1 mx-2">
        <Link to='/register' className="text-secondary">
          INSCRIPTION
          <Icon icon="arrow-right" className="ml-2" />
        </Link>
      </h5>
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
        <Form.Label><small><b>MOT DE PASSE</b></small></Form.Label>
        <span className="underline" />
        <div className="highlight" />
      </Form.Group>

      <AutoForm.Submit
        variant="secondary"
        className="d-block px-5 my-2 mx-auto"
      >Connexion</AutoForm.Submit>
    </>
  );

}


const Login = Unauthenticated(() => {

  const { login, error } = useAuth();
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

            <SimpleError error={error} />

            <LoginForm />
            <hr />

            <Link to="/restore" className="d-block text-center text-light">
              Oublié votre mot de passe?
            </Link>
          </AutoForm>

        </Col>
      </Row>
    </Container>
  );
});


export default Login;
