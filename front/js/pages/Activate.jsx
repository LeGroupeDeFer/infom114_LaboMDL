import React, { useState, useEffect } from 'react';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import Spinner from 'react-bootstrap/Spinner';
import { useParams, useHistory, Link } from 'react-router-dom';
import { faMailBulk } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import api from '../lib/api';

function ActivationStatus({ status }) {

  let Status;

  switch (status) {
    case "await":
      Status = (
        <>
          <h4>Awaiting activation...</h4>
          <Spinner animation="border" role="status">
            <span className="sr-only">Loading...</span>
          </Spinner>
        </>
      );
      break;
    case "refused":
      Status = (
        <>
          <h4 className="text-danger"><b>Failure</b></h4>
          <p><b>
            The code you provided is either invalid or has expired. If your
            account is still not activated, you can attempt to
            <Link to="/login" className="text-secondary mx-1">login</Link>to
            generate a new activation link.
          </b></p>
        </>
      );
      break;

    case "accepted":
      Status = (
        <>
          <h4 className="text-success"><b>Success</b></h4>
          <p><b>
            Your account activation was successful!. Head to the
            <Link to="/login" className="text-secondary mx-1">login page</Link>
            to start participating!
          </b></p>
        </>
      );
      break;

    default:
      Status = (
        <>
          <h4 className="text-secondary"><b>Almost done...</b></h4>
          <p><b>
            Please click on the link that has just been sent to your email
            account to verify your email and continue the registration process.
          </b></p>
        </>
      );
  }

  return Status;
}

export default function Activate() {

  let history = useHistory();
  let { id, token } = useParams();
  let [activation, setActivation] = useState("await");

  useEffect(() => {
    if (!id || !token) {
      setActivation("pending");
      return;
    }

    let isSubscribed = true;

    api.activate(id, token)
      .then(() => {
        if (isSubscribed)
          setActivation("accepted");
      })
      .catch(() => {
        if (isSubscribed)
          setActivation("refused");
      });

    return () => isSubscribed = false;
  }, [])

  return (
    <Container className="activation-content text-center">
      <Row>
        <Col className="text-secondary">
          <h1>
            <Icon icon={faMailBulk} size="1x" className="mr-3" />
            <b>Activation</b>
          </h1>
          <hr />
        </Col>
      </Row>
      <Row>
        <Col md={{ span: 6, offset: 3 }}>
          <ActivationStatus status={activation} />
        </Col>
      </Row>
      <Row>
        <Col>
          <hr />
          <Link to="/" className="text-secondary mr-2"><b>Home</b></Link>
          <Link to="/login" className="text-secondary mr-2"><b>Login</b></Link>
          <Link to="/faq" className="text-secondary"><b>FAQ</b></Link>
        </Col>
      </Row>
    </Container >
  );

}