import clsx from 'clsx';
import React from 'react';
import Button from 'react-bootstrap/Button';
import { useForm } from './formContext';

/**
 * @memberof AutoForm
 * @param {Object} props
 */
function Submit({ className, ...others }) {

  const { validity, error, submit } = useForm();
  /* istanbul ignore next */
  const cls = clsx(className, error && 'btn-danger')

  return (
    <Button
      {...others}
      type="submit"
      disabled={!validity}
      className={cls}
    />
  );

}


export default Submit;
