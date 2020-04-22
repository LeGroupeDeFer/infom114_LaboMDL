import React, { useState, useEffect } from 'react';
import { Container, Row, Col, Spinner } from 'react-bootstrap';
import { useParams, useHistory, Link } from 'react-router-dom';
import { faMailBulk } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { useRequest } from '../hooks';
import { Dialog, Unauthenticated } from '../components';
import { api, trace } from '../lib';


function ActivationStatus() {

  const { id, token } = useParams();
  const [error, success] = useRequest(api.auth.activate, [id, token]);

  if (error)
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

  if (success)
    return (
      <>
        <h4 className="text-success"><b>Success</b></h4>
        <p><b>
          Your account activation was successful!. Head to the
          <Link to="/login" className="text-secondary mx-1">login page</Link>
          to start participating!
        </b></p>
      </>
    );

  return (
    <>
      <h4>Awaiting activation...</h4>
      <Spinner animation="border" role="status">
        <span className="sr-only">Loading...</span>
      </Spinner>
    </>
  );

}


const Activate() = Unauthenticated(() => {
  return (
    <Dialog icon={faMailBulk} title="Activation">
      <ActivationStatus />
    </Dialog>
  );
});


export default Activate;
