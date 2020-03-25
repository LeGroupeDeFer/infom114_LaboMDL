import clsx from 'clsx';
import { bool, string } from 'prop-types';
import React, { useEffect, useState } from 'react';
import Form from 'react-bootstrap/Form';
import { useForm } from './formContext';


/**
 * @memberOf AutoForm
 * @component
 * @param {Object} props
 * @returns JSX.Element
 */
function Switch({ optional, name, className, variant, ...others }) {

  const localValidator = value => optional ? true : value;
  const [toggled, setToggled] = useState(false);

  const { register, onChange, error } = useForm();
  useEffect(() => register(name, toggled, optional), []);

  const localOnChange = event => {
    setToggled(toggled => !toggled);
    onChange(name, !toggled, localValidator(!toggled));
  };

  /* istanbul ignore next */
  const cls = clsx(className, `custom-switch-${error ? 'danger' : variant}`);

  return (
    <Form.Check
      {...others}
      type="switch"
      onChange={localOnChange}
      className={cls}
    />
  )
}

Switch.defaultProps = {
  variant: 'primary'
};

Switch.propTypes = {
  name: string.isRequired,
  optional: bool,
  className: string,
  variant: string,
};


export default Switch;
