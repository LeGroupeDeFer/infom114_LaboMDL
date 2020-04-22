import React, { useState, useEffect } from 'react';
import AutoForm from '../components/AutoForm';
import { Container, Row, Col, Form } from 'react-bootstrap';
import { useParams, useHistory, Link } from 'react-router-dom';
import { faUserLock } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { useRequest, useAction } from '../hooks';
import { Dialog, Authenticated } from '../components';
import { Action, api, trace } from '../lib';
import { isUnamurEmail } from '../lib/validators';


function RestoreForm() {
  const [onSubmit, error, success] = useAction(
    ({ email }) => api.auth.restore(email)
  );

  if (success)
    return (
      <>
        <h4 className="text-secondary"><b>Almost done...</b></h4>
        <p><b>
          Please click on the link that has just been sent to your mailbox to
          proceed.
        </b></p>
      </>
    );

  if (error)
    return (
      <>
        <h4 className="text-danger"><b>Failure</b></h4>
        <p><b>
          The code you provided is either invalid or has expired.
        </b></p>
      </>
    );

  return (
    <AutoForm onSubmit={onSubmit}>
      <Form.Group className='form-group-material'>
        <AutoForm.Control
          id="email"
          name="email"
          type="text"
          eraseOnFailure={false}
          validator={isUnamurEmail}
        />
        <Form.Label>
          <small><p>EMAIL</p></small>
        </Form.Label>
        <span className="underline" />
        <div className="highlight" />
      </Form.Group>
      <AutoForm.Submit
        variant="secondary"
        className="d-block px-5 my-2 mx-auto"
      >
        Submit
      </AutoForm.Submit>
    </AutoForm>
  );

}


const Restore = Unauthenticated(() => {
  return (
    <Dialog icon={faUserLock} title="Account restoration">
      <RestoreForm />
    </Dialog>
  );
});


export default Restore;
