import React from 'react';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import clsx from 'clsx';
import { emptyObject } from 'unanimity/lib';

export function Simple({ error, className }) {

  const cls = clsx('bg-dark text-center text-light p-2', className);

  return error && !emptyObject(error) ? (
    <div className={cls}>
      <Icon
        icon="exclamation-circle"
        className="text-danger mr-2"
        style={{ display: 'inline-box' }}
      />
      <p className="m-0">{error.reason}</p>
    </div>
  ) : <></>;
}

Simple.defaultProps = {
  className: ''
}