import { faArrowLeft, faArrowRight } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon as Icon } from "@fortawesome/react-fontawesome";
import React, { useEffect } from "react";
import Col from "react-bootstrap/Col";
import Container from "react-bootstrap/Container";
import Form from "react-bootstrap/Form";
import Row from "react-bootstrap/Row";
import { Link, useHistory } from "react-router-dom";
import AutoForm from "../components/AutoForm";
import Flexbox from "../components/Flexbox";
import Image from '../components/Image';
import { useAuth } from "../context/authContext";
import { isUnamurEmail, isValidNatural, isValidPassword } from "../lib/validators";


function Header() {
  return (
    <Flexbox justify="center" align="end" className="mb-3 form-header">
      <h4 className="mb-1 mx-2">
        <Link to="/" className="text-secondary">
          <Icon icon={faArrowLeft} className="mr-2" />
          HOME
        </Link>
      </h4>
      <h1 className="mb-0 mx-2">SIGN UP</h1>
      <h4 className="mb-1 mx-2">
        <Link to="/login" className="text-secondary">
          SIGN IN
          <Icon icon={faArrowRight} className="ml-2" />
        </Link>
      </h4>
    </Flexbox>
  )
}

function RegisterForm() {

  return (
    <>
      {/* Firstname & Lastname */}
      <Row>

        <Col sm="6">
          <Form.Group className='form-group-material'>
            <AutoForm.Control
              id="firstname"
              name="firstname"
              type="text"
              eraseOnFailure={false}
            />
            <Form.Label>
              <small><b>FIRSTNAME</b></small>
            </Form.Label>
            <span className="underline" />
            <div className="highlight" />
          </Form.Group>
        </Col>
        <Col>
          <Form.Group className='form-group-material'>
            <AutoForm.Control
              id="lastname"
              name="lastname"
              type="text"
              eraseOnFailure={false}
            />
            <Form.Label>
              <small><b>LASTNAME</b></small>
            </Form.Label>
            <span className="underline" />
            <div className="highlight" />
          </Form.Group>
        </Col>

      </Row>

      {/* Email & Phone */}
      <Row>

        <Col sm="6">
          <Form.Group className='form-group-material'>
            <AutoForm.Control
              id="email"
              name="email"
              type="email"
              validator={isUnamurEmail}
            />
            <Form.Label>
              <small><b>EMAIL</b></small>
            </Form.Label>
            <span className="underline" />
            <div className="highlight" />
          </Form.Group>
        </Col>
        <Col>
          <Form.Group className='form-group-material'>
            <AutoForm.Control
              id="phone"
              name="phone"
              type="tel"
            />
            <Form.Label>
              <small><b>PHONE</b></small>
            </Form.Label>
            <span className="underline" />
            <div className="highlight" />
          </Form.Group>
        </Col>

      </Row>

      {/* Street, Number & Box */}
      <Row>

        <Col xs="12" sm="7">
          <Form.Group className='form-group-material'>
            <AutoForm.Control
              id="street"
              name="street"
              type="text"
              eraseOnFailure={false}
            />
            <Form.Label>
              <small><b>STREET</b></small>
            </Form.Label>
            <span className="underline" />
            <div className="highlight" />
          </Form.Group>
        </Col>
        <Col xs="6" sm="3">
          <Form.Group className='form-group-material'>
            <AutoForm.Control
              id="number"
              name="number"
              type="number"
              min="0"
              eraseOnFailure={false}
              className="flex-grow-1"
              validator={isValidNatural}
            />
            <Form.Label>
              <small><b>NUMBER</b></small>
            </Form.Label>
            <span className="underline" />
            <div className="highlight" />
          </Form.Group>
        </Col>
        <Col xs="6" sm="2">
          <Form.Group className='form-group-material'>
            <AutoForm.Control
              optional
              id='box'
              name="box"
              type="text"
              eraseOnFailure={false}
              className="flex-grow-1"
            />
            <Form.Label>
              <small><b>BOX</b></small>
            </Form.Label>
            <span className="underline" />
            <div className="highlight" />
          </Form.Group>
        </Col>

      </Row>

      {/* Zip & City */}
      <Row>

        <Col>
          <Form.Group className='form-group-material'>
            <AutoForm.Control
              id="zip"
              name="zip"
              min="0"
              type="number"
              eraseOnFailure={false}
              className="flex-grow-1"
              validator={isValidNatural}
            />
            <Form.Label>
              <small><b>ZIP CODE</b></small>
            </Form.Label>
            <span className="underline" />
            <div className="highlight" />
          </Form.Group>
        </Col>
        <Col>
          <Form.Group className='form-group-material'>
            <AutoForm.Control
              id="city"
              name="city"
              type="text"
              eraseOnFailure={false}
              className='flex-grow-4'
            />
            <Form.Label>
              <small><b>CITY</b></small>
            </Form.Label>
            <span className="underline" />
            <div className="highlight" />
          </Form.Group>
        </Col>

      </Row>

      {/* Password and confirmation */}
      <Row>

        <Col xs="12" sm="6">
          <Form.Group className='form-group-material'>
            <AutoForm.Control
              id="password"
              name="password"
              type="password"
              eraseOnFailure={true}
              validator={isValidPassword}
            />
            <Form.Label>
              <small><b>PASSWORD</b></small>
            </Form.Label>
            <span className="underline" />
            <div className="highlight" />
          </Form.Group>
        </Col>
        <Col>
          <Form.Group className='form-group-material'>
            <AutoForm.Control
              id="confirm_password"
              name="confirm_password"
              type="password"
              eraseOnFailure={true}
              validator={isValidPassword}
            />
            <Form.Label>
              <small><b>CONFIRM</b></small>
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
          label="I accept the general terms and conditions"
          className="mx-auto my-3"
        />
      </Row>

      {/* Submit*/}
      <Row>
        <AutoForm.Submit
          variant="secondary"
          className="d-block px-5 my-2 mx-auto"
        >
          Register
        </AutoForm.Submit>
      </Row>

    </>

  )
}

function Register(props) {
  const { register, user } = useAuth();
  const history = useHistory()
  if (user) {
    history.push('/');
    // Shouldn't get here except in testing
    return <></>;
  }

  const handleSubmit = newUser =>
    register(newUser).then(_ => history.push('/login'));

  useEffect(() => (user ? history.replace("/") : undefined), [user])

  return (
    <Container className="register-form">
      <Header />
      <hr />

      <Row>
        <Col lg="6" sm="0">
          <Image
            cover
            path="https://placehold.it/500x500"
          />
        </Col>
        <Col>
          <AutoForm onSubmit={handleSubmit} autoComplete="off">
            <RegisterForm />
          </AutoForm>
        </Col>
      </Row>

    </Container >
  );
}

export default Register
