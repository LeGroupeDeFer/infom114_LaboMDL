import React from 'react';
import { Row, Col, Form } from 'react-bootstrap';
import { useParams, Link } from 'react-router-dom';
import { faUserLock } from '@fortawesome/free-solid-svg-icons';

import { Dialog, AutoForm, Unauthenticated } from '../components';
import { useAction } from '../hooks';
import api from '../lib/api';


function RecoveryForm({ id, token }) {

  const [handler, error, success] = useAction(
    ({ password, confirm }) => {
      if (password !== confirm)
        return Promise.reject({
          code: 0,
          reason: 'Password and confirmation are differents'
        });
      return api.auth.recover(id, password, token);
    }
  );

  if (error)
    return (
      <>
        <h4 className="text-danger"><b>Failure</b></h4>
        <p><b>
          The token you provided is either invalid or has expired. If you still
          wish to recover your account, you may head to
          <Link to="/restore" className="text-secondary mx-1">this page</Link>to
          generate a new recovery link.
        </b></p>
      </>
    );

  if (success)
    return (
      <>
        <h4 className="text-success"><b>Success</b></h4>
        <p><b>
          Your account password was changed!. Head to the
          <Link to="/login" className="text-secondary mx-1">login page</Link>
          to start participating!
        </b></p>
      </>
    );

  return (
    <AutoForm onSubmit={handler}>

      <Row>
        <Col>
          <Form.Group className='form-group-material'>
            <AutoForm.Control
              id="password"
              name="password"
              type="password"
              eraseOnFailure={true}
            />
            <Form.Label>
              <small><p>NEW PASSWORD</p></small>
            </Form.Label>
            <span className="underline" />
            <div className="highlight" />
          </Form.Group>
        </Col>
      </Row>

      <Row>
        <Col>
          <Form.Group className='form-group-material'>
            <AutoForm.Control
              id="confirm"
              name="confirm"
              type="password"
              eraseOnFailure={true}
            />
            <Form.Label>
              <small><p>CONFIRM PASSWORD</p></small>
            </Form.Label>
            <span className="underline" />
            <div className="highlight" />
          </Form.Group>
        </Col>
      </Row>

      <Row>
        <Col>
          <AutoForm.Submit
            variant="secondary"
            className="d-block px-5 my-2 mx-auto"
          >
            Submit
          </AutoForm.Submit>
        </Col>
      </Row>

    </AutoForm>
  );

}


const Recover = Unauthenticated(() => {
  const { id, token } = useParams();

  return (
    <Dialog icon={faUserLock} title="Recovery">
      <RecoveryForm id={id} token={token} />
    </Dialog>
  );
});


export default Recover;
