import React, { useState, useEffect } from 'react';
import { Spinner } from 'react-bootstrap';
import { useParams, Link } from 'react-router-dom';
import { usePositiveEffect } from '../hooks';
import { Dialog, Unauthenticated } from '../components';
import { api } from '../lib';


function ActivationStatus({ id, token }) {

  const [promise, setPromise] = useState(null);
  const [error, setError] = useState(null);
  const [success, setSuccess] = useState(null);

  useEffect(() => setPromise(api.auth.activate(id, token)), []);

  usePositiveEffect(() => {
    let isSubscribed = true;
    promise
      .then(() => isSubscribed ? setSuccess(true) : undefined)
      .catch(error => isSubscribed ? setError(error) : undefined)
      .finally(() => setPromise(null));
    return () => isSubscribed = false;
  }, [promise]);

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


const Activate = Unauthenticated(() => {
  const { id, token } = useParams();
  return (
    <Dialog icon="mail-bulk" title="Activation">
      <ActivationStatus id={id} token={token} />
    </Dialog>
  );
});


export default Activate;
