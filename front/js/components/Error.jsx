import React from 'react';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { faExclamationCircle } from '@fortawesome/free-solid-svg-icons';
import clsx from 'clsx';


export function Simple({ error, className }) {

  const cls = clsx('bg-dark text-center text-light p-2', className);

  return error ? (
    <div className={cls}>
      <Icon
        icon={faExclamationCircle}
        className="text-danger mr-2"
        style={{ display: 'inline-box' }}
      />
      <p className="m-0">{error.reason}</p>
    </div>
  ) : false;
}

Simple.defaultProps = {
  className: ''
}