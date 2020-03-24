import React, { Suspense } from 'react';
import Spinner from 'react-bootstrap/Spinner';


// DefaultSpinner :: None => Component
const DefaultSpinner = _ => (
  <Spinner
    animation='grow'
    role='status'
    className='abs-center default-spinner'
  >
    <span className="sr-only">Loading...</span>
  </Spinner>
);

// Waiting :: (Component, Component?) => Object => Component
const Waiting = (Component, Spinner = DefaultSpinner) => props => (
  <Suspense fallback={<Spinner />}>
    <Component {...props} />
  </Suspense>
);

Waiting.DefaultSpinner = DefaultSpinner;


export default Waiting;