import clsx from 'clsx';
import React from 'react';
import Button from 'react-bootstrap/Button';
import { useValidation } from './validationContext';

/**
 * @component
 * @memberof AutoForm
 * @param {Object} props
 */
function Submit({ className, ...others }) {
  const { validated, error } = useValidation();
  return (
    <Button
      type="submit"
      disabled={!validated}
      className={clsx(className, error && 'btn-danger')}
      {...others}
    />
  );
}


export default Submit;
