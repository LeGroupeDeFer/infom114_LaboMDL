import React, { useState } from 'react';
import AutoForm from '../components/AutoForm';
import { Form } from 'react-bootstrap';
import {usePositiveEffect} from '../hooks';
import { Dialog, Unauthenticated } from '../components';
import { api } from '../lib';
import { isUnamurEmail } from '../lib/validators';


function RestoreForm() {
  const [{ error, success, request }, setState] = useState({
    error: null, success: null, request: null
  });

  const onSubmit = ({ email }) => {
    setState(s => ({ ...s, request: api.auth.restore(email) }));
  };

  usePositiveEffect(() => {
    request
      .then(() => setState(s => ({ ...s, success: true, error: false })))
      .catch(() => setState(s => ({ ...s, success: false, error: true })))
      .finally(() => setState(s => ({ ...s, request: null })));
  }, [request])

  if (success)
    return (
      <>
        <h4 className="text-secondary"><b>Presque...</b></h4>
        <p><b>
          Veuillez cliquer sur le lien qui vient d'être envoyé à votre boîte email pour continuer.
        </b></p>
      </>
    );

  if (error)
    return (
      <>
        <h4 className="text-danger"><b>Failure</b></h4>
        <p><b>
          Le code fourni est invalide ou a expiré.
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
        Soumettre
      </AutoForm.Submit>
    </AutoForm>
  );

}


const Restore = Unauthenticated(() => {
  return (
    <Dialog icon="user-lock" title="Récupération de compte">
      <RestoreForm />
    </Dialog>
  );
});


export default Restore;
