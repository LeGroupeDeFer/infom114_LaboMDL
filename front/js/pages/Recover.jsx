import React, {useState} from 'react';
import { Row, Col, Form } from 'react-bootstrap';
import { useParams, Link } from 'react-router-dom';

import { Dialog, AutoForm, Unauthenticated } from '../components';
import {useAction, usePositiveEffect} from '../hooks';
import api from '../lib/api';


function RecoveryForm({ id, token }) {


  const [{ request, error, success }, setState] = useState({
    request: null, error: null, success: null
  });

  const onSubmit = ({ password, confirm }) => {
    if (password !== confirm)
      return Promise.reject({
        code: 0,
        reason: 'Les mots de passe diffèrent'
      });
    setState(s => ({ ...s, request: api.auth.recover(id, password, token) }));
  };

  usePositiveEffect(() => {
    request
      .then(() => setState(s => ({ ...s, success: true, error: false })))
      .catch(() => setState(s => ({ ...s, success: false, error: true })))
      .finally(() => setState(s => ({ ...s, request: null })));
  }, [request])

  if (error)
    return (
      <>
        <h4 className="text-danger"><b>Failure</b></h4>
        <p><b>
          Le code fourni est invalide ou a expiré. Si vous desirez toujours récuperer votre compte,
          vous pouvez visiter
          <Link to="/restore" className="text-secondary mx-1">cette page</Link> pour
          génerer une nouveau lien de récupération.
        </b></p>
      </>
    );

  if (success)
    return (
      <>
        <h4 className="text-success"><b>Succès!</b></h4>
        <p><b>
          Le mot de passe de votre compte a changé! Dirigez vous vers la
          <Link to="/login" className="text-secondary mx-1">page d'authentification</Link>
          pour commencer à participer!
        </b></p>
      </>
    );

  return (
    <AutoForm onSubmit={onSubmit}>

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
              <small><p>NOUVEAU MOT DE PASSE</p></small>
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
              <small><p>CONFIRMER</p></small>
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
            Soumettre
          </AutoForm.Submit>
        </Col>
      </Row>

    </AutoForm>
  );

}


const Recover = Unauthenticated(() => {
  const { id, token } = useParams();

  return (
    <Dialog icon="user-lock" title="Recovery">
      <RecoveryForm id={id} token={token} />
    </Dialog>
  );
});


export default Recover;
