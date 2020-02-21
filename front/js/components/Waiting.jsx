import React, { Suspense } from 'react';
import Spinner from 'react-bootstrap/Spinner';

const CenteredSpinner = _ => (
  <Spinner
    animation='grow'
    role='status'
    className='abs-center'
  >
    <span className="sr-only">Loading...</span>
  </Spinner>
);

export default function Waiting(Component) {
  return props => (
    <Suspense fallback={<CenteredSpinner />}>
      <Component {...props} />
    </Suspense>
  )
}

Waiting.Spinner = CenteredSpinner;