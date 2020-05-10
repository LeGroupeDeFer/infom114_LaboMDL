import React from 'react';
import AutoForm from '../components/AutoForm';
import { Form } from 'react-bootstrap';
import { useAction } from '../hooks';
import { Dialog, Unauthenticated } from '../components';
import { api } from '../lib';
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
    <Dialog icon="user-lock" title="Account restoration">
      <RestoreForm />
    </Dialog>
  );
});


export default Restore;
