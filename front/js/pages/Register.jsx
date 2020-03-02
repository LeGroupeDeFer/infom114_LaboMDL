import React, { useEffect, useState } from "react"
import { useHistory, Link } from "react-router-dom"
import { useAuth } from "../context/authContext"
import Container from "react-bootstrap/Container"
import Form from "react-bootstrap/Form"
import Flexbox from "../components/Flexbox"
import AutoForm from "../components/AutoForm"
import Row from "react-bootstrap/Row"
import Col from "react-bootstrap/Col"
import { isUnamurEmail, isValidPassword } from "../utils/validators"
import { FontAwesomeIcon as Icon } from "@fortawesome/react-fontawesome"
import { faArrowLeft, faArrowRight } from "@fortawesome/free-solid-svg-icons"

function Header(props) {
  return (
    <Flexbox justify="center" align="end" className="mb-3">
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
      <Form.Group controlId="registerEmail">
        <Form.Label>
          <small>
            <b>EMAIL</b>
          </small>
        </Form.Label>
        <AutoForm.Control
          required
          name="email"
          type="email"
          placeholder="you@unamur.be"
          validator={isUnamurEmail}
        />
      </Form.Group>

      <Form.Group controlId="registerPassword">
        <Form.Label>
          <small>
            <b>PASSWORD</b>
          </small>
        </Form.Label>
        <AutoForm.Control
          required
          name="password"
          type="password"
          eraseOnFailure={true}
          validator={isValidPassword}
        />
      </Form.Group>
      <Form.Group controlId="retypePassword">
        <Form.Label>
          <small>
            <b>RETYPE PASSWORD</b>
          </small>
        </Form.Label>
        <AutoForm.Control
          required
          name="retype_password"
          type="password"
          eraseOnFailure={true}
          validator={isValidPassword}
        />
      </Form.Group>

      <Form.Group controlId="registerFirstName">
        <Form.Label>
          <small>
            <b>FIRSTNAME</b>
          </small>
        </Form.Label>
        <AutoForm.Control
          required
          name="firstname"
          type="text"
          eraseOnFailure={false}
        />
      </Form.Group>

      <Form.Group controlId="registerLastName">
        <Form.Label>
          <small>
            <b>LASTNAME</b>
          </small>
        </Form.Label>
        <AutoForm.Control
          required
          name="lastname"
          type="text"
          eraseOnFailure={false}
        />
      </Form.Group>

      <AutoForm.Submit
        variant="secondary"
        className="d-block px-5 my-2 mx-auto"
      >
        Register
      </AutoForm.Submit>
    </>
  )
}

function Register(props) {
  const { register, user } = useAuth()
  const history = useHistory()
  useEffect(() => (user ? history.replace("/") : undefined), [user])
  const [error, setError] = useState(false)

  const handleSubmit = data => {
    // const { email, password } = data
    // register(data)
    //   .then(_ => history.push("/"))
    //   .catch(error => setError(error))
    console.log("bite")
  }

  return (
    <Container className="register-form">
      <Row>
        <Col lg={{ span: 6, offset: 3 }}>
          <AutoForm error={error} onSubmit={handleSubmit}>
            <Header />
            <hr />

            <RegisterForm />
          </AutoForm>
        </Col>
      </Row>
    </Container>
  )
}

export default Register
