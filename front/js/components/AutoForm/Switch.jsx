import clsx from 'clsx';
import { bool, string } from 'prop-types';
import React, { useEffect, useState } from 'react';
import Form from 'react-bootstrap/Form';
import { useValidation } from './validationContext';


/**
 * @memberOf AutoForm
 * @component
 * @param {Object} props
 * @returns JSX.Element
 */
function Switch({ optional, name, className, variant, ...others }) {

  const [toggled, setToggled] = useState(false);
  const localValidator = value => optional ? true : value;
  const { binding, failure } = useValidation();
  const onChange = binding(name);
  useEffect(() => onChange({ value: toggled, isValid: optional }), []);

  const localOnChange = event => {
    setToggled(toggled => !toggled);
    onChange({ value: !toggled, isValid: localValidator(!toggled) });
  };

  const cls = clsx(className, `custom-switch-${variant}`);

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
