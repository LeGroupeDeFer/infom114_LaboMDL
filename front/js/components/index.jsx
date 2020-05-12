export { default as AutoForm } from './AutoForm';
export { default as Circle } from './Circle';
export { default as Flexbox } from './Flexbox';
export { default as Image } from './Image';
export { default as Waiting } from './Waiting';
export { Unauthenticated, Authenticated, Dialog } from './Auth';
export { default as SearchBar } from './SearchBar';
export { default as Post } from './Post';
export { Simple } from './Error';
export { default as Moment } from './Moment';

import React from 'react';
import Spinner from 'react-bootstrap/Spinner';

export function Loading() {
  return (
    <div className="abs-center">
      <Spinner
        animation="border"
        variant="primary"
        role="status"
      />
    </div>
  );
}