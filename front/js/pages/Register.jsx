import React, { useState } from 'react';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { Container, Row, Col, Form } from 'react-bootstrap';
import { Link } from 'react-router-dom';

import { AutoForm, Flexbox, Image, Unauthenticated } from '../components';
import { subscribed, usePositiveEffect } from '../hooks';
import { api, aggregate } from '../lib';
import { isUnamurEmail, isValidNatural, isValidPassword, isValidPhoneNumber } from '../lib/validators';


const confirmPassword = ({ password, confirmPassword }) =>
  password && confirmPassword && password.value === confirmPassword.value;


function Header() {
  return (
    <Flexbox justify="center" align="end" className="mb-3 form-header">
      <h5 className="mb-1 mx-2">
        <Link to="/" className="text-secondary">
          <Icon icon="arrow-left" className="mr-2" />
          ACCUEIL
        </Link>
      </h5>
      <h2 className="mb-0 mx-2">INSCRIPTION</h2>
      <h5 className="mb-1 mx-2">
        <Link to="/login" className="text-secondary">
          CONNEXION
          <Icon icon="arrow-right" className="ml-2" />
        </Link>
      </h5>
    </Flexbox>
  );
}


function RegisterForm() {
  return (
    <>
      {/* Firstname & Lastname */}
      <Row>

        <Col sm="6">
          <Form.Group className="form-group-material">
            <AutoForm.Control
              id="firstname"
              name="firstname"
              type="text"
            />
            <Form.Label>
              <small><b>PRENOM*</b></small>
            </Form.Label>
            <span className="underline" />
            <div className="highlight" />
          </Form.Group>
        </Col>
        <Col>
          <Form.Group className="form-group-material">
            <AutoForm.Control
              id="lastname"
              name="lastname"
              type="text"
            />
            <Form.Label>
              <small><b>NOM*</b></small>
            </Form.Label>
            <span className="underline" />
            <div className="highlight" />
          </Form.Group>
        </Col>

      </Row>

      {/* Email & Phone */}
      <Row>

        <Col sm="6">
          <Form.Group className="form-group-material">
            <AutoForm.Control
              id="email"
              name="email"
              type="email"
              validator={isUnamurEmail}
            />
            <Form.Label>
              <small><b>EMAIL*</b></small>
            </Form.Label>
            <span className="underline" />
            <div className="highlight" />
          </Form.Group>
        </Col>
        <Col>
          <Form.Group className="form-group-material">
            <AutoForm.Control
              id="phone"
              name="phone"
              type="tel"
              validator={isValidPhoneNumber}
            />
            <Form.Label>
              <small><b>TÉLEPHONE*</b></small>
            </Form.Label>
            <span className="underline" />
            <div className="highlight" />
          </Form.Group>
        </Col>

      </Row>

      {/* Street, Number & Box */}
      <Row>

        <Col xs="12" sm="6">
          <Form.Group className="form-group-material">
            <AutoForm.Control
              optional
              id="street"
              name="street"
              type="text"
            />
            <Form.Label>
              <small><b>RUE</b></small>
            </Form.Label>
            <span className="underline" />
            <div className="highlight" />
          </Form.Group>
        </Col>
        <Col xs="6" sm="3">
          <Form.Group className="form-group-material">
            <AutoForm.Control
              optional
              id="number"
              name="number"
              type="number"
              min="0"
              className="flex-grow-1"
              validator={isValidNatural}
            />
            <Form.Label>
              <small><b>NUMÉRO</b></small>
            </Form.Label>
            <span className="underline" />
            <div className="highlight" />
          </Form.Group>
        </Col>
        <Col xs="6" sm="3">
          <Form.Group className="form-group-material">
            <AutoForm.Control
              optional
              id="boxNumber"
              name="boxNumber"
              type="text"
              className="flex-grow-1"
            />
            <Form.Label>
              <small><b>BOÎTE</b></small>
            </Form.Label>
            <span className="underline" />
            <div className="highlight" />
          </Form.Group>
        </Col>

      </Row>

      {/* Zip & City */}
      <Row>

        <Col>
          <Form.Group className="form-group-material">
            <AutoForm.Control
              optional
              id="zipcode"
              name="zipcode"
              min="0"
              type="text"
              className="flex-grow-1"
              validator={isValidNatural}
            />
            <Form.Label>
              <small><b>CODE POSTAL</b></small>
            </Form.Label>
            <span className="underline" />
            <div className="highlight" />
          </Form.Group>
        </Col>
        <Col>
          <Form.Group className="form-group-material">
            <AutoForm.Control
              optional
              id="city"
              name="city"
              type="text"
              className='flex-grow-4'
            />
            <Form.Label>
              <small><b>VILLE</b></small>
            </Form.Label>
            <span className="underline" />
            <div className="highlight" />
          </Form.Group>
        </Col>

      </Row>

      {/* Password and confirmation */}
      <Row>

        <Col xs="12" sm="6">
          <Form.Group className="form-group-material">
            <AutoForm.Control
              eraseOnFailure
              id="password"
              name="password"
              type="password"
              validator={isValidPassword}
            />
            <Form.Label>
              <small><b>MOT DE PASSE*</b></small>
            </Form.Label>
            <span className="underline" />
            <div className="highlight" />
          </Form.Group>
        </Col>
        <Col>
          <Form.Group className="form-group-material">
            <AutoForm.Control
              eraseOnFailure
              id="confirmPassword"
              name="confirmPassword"
              type="password"
              validator={isValidPassword}
            />
            <Form.Label>
              <small><b>CONFIRMATION*</b></small>
            </Form.Label>
            <span className="underline" />
            <div className="highlight" />
          </Form.Group>
        </Col>

      </Row>

      {/* Terms */}
      <Row>
        <AutoForm.Switch
          variant="secondary"
          id="terms"
          name="terms"
          label="J'accepte les conditions générales d'utilisation"
          className="mx-auto my-3"
        />
      </Row>

      {/* Submit*/}
      <Row>
        <AutoForm.Submit
          variant="secondary"
          className="d-block px-5 my-2 mx-auto"
        >
          S'enregistrer
        </AutoForm.Submit>
      </Row>

    </>
  );
}


function RegistrationSuccess({ email }) {
  return (
    <Flexbox className="h-100 text-center" align="center" direction="column" justify="center">
      <Icon icon="check-circle" className="display-4 pb-3" />
      <h1 className="pb-3 text-secondary font-weight-bold">
        Succès
      </h1>
      <p>
        Nous vous avons envoyé un email à <i className="px-1 text-primary bg-light">{email}</i>.
        Suivez les instructions dans ce message pour poursuivre le processus d'enregistrement.
      </p>
    </Flexbox>
  );
}


const EmailConflict = ({ className }) => (
  <p className={className}>
    Ce compte existe déjà, avez vous
    <Link to="/restore" className="text-primary"> oublié votre mot de passe</Link> ?
  </p>
);

const InvalidInfo = ({ className }) => (
  <p className={className}>Ces informations sont invalides</p>
);

const errorMessages = {
  409: EmailConflict,
  422: InvalidInfo
}

function ErrorMessage({ error }) {
  if (!error)
    return <></>;

  const LocalError = errorMessages[error.code];

  return (
    <div className="bg-dark text-center p-2">
      <Icon
        icon="exclamation-circle"
        className="text-danger mr-2"
        style={{ display: 'inline-box' }}
      />
      <LocalError className="m-0" />
    </div>
  );
}


const Register = Unauthenticated((props) => {

  const [state, setState] = useState({ email: null, error: null, success: null, promise: null });

  const onSubmit = info => {
    const data = aggregate(
      info,
      'address',
      ['street', 'number', 'boxNumber', 'zipcode', 'city']
    );

    // If one field of the address is empty, remove thde whole address
    // FIXME - Should show an error message to the user instead
    if (Object.values(data.address).some(x => !x))
      delete(data.address);

    setState(s => ({
      ...s,
      email: data.email,
      promise: api.auth.register(data)
    }));
  }

  usePositiveEffect(subscribed(
    state.promise,
    () => setState(s => ({ ...s, success: true, error: false, promise: null })),
    error => setState(s => ({ ...s, success: false, error, promise: null })),
  ), [state.promise]);

  const { email, error, success } = state;

  return (
    <Container className="register-form">
      <AutoForm
        onSubmit={onSubmit}
        validator={confirmPassword}
        autoComplete="off"
      >
        <Header />
        <hr />

        <Row>
          <Col>
            <ErrorMessage error={error} />
            { success ? <RegistrationSuccess email={email} /> : <RegisterForm /> }
          </Col>
          <Col lg="6" sm="0">
            <Image cover
              path="https://placehold.it/500x500"
              width="500px"
              height="500px"
              float="right"
              className="d-none d-lg-block"
            />
          </Col>
        </Row>
      </AutoForm>
    </Container >
  );

});


export default Register;
