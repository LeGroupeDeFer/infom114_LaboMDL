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

  if (status === null)
    return (
      <>
        <h5>Awaiting activation...</h5>
        <Spinner animation="border" role="status">
          <span className="sr-only">Loading...</span>
        </Spinner>
      </>
    );

  if (!status)
    return (
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

  return (
    <>
      <h4 className="text-success"><b>Success</b></h4>
      <p><b>
        Your account activation was successful!. Head to the
        <Link to="/login" className="text-secondary mx-1">login page</Link>
        and start participating!
      </b></p>
    </>
  );

}

export default function Recover() {

  

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
          <ActivationStatus status={activated} />
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