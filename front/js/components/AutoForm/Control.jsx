import clsx from 'clsx';
import { string } from 'prop-types';
import React, { useEffect, useState } from 'react';
import Form from 'react-bootstrap/Form';
import { useForm } from './formContext';

/* istanbul ignore next */
function cast(value, type) {
  switch (type) {
    case 'checkbox':
    case 'radio':
      return Boolean(value);
    case 'number':
      return Number(value);
    case 'date':
      return Date(value);
    case 'text':
    case 'email':
    case 'tel':
    case 'password':
    case 'url':
      return String(value);
    default:
      return value;
  }
}


/**
 * [Control]{@link AutoForm.Control} custom validator. Receives a casted input value, returns **true** when said input is valid, **false** otherwise.
 * 
 * @callback Validator
 * @memberof AutoForm
 * 
 * @param { any } value The input value to validate.
 * @returns { boolean } The validation status.
 */

/**
 * [Control]{@link AutoForm.Control} properties shape.
 * @typedef { Object } ControlProps
 * @memberof AutoForm
 *
 * @property { string }     name The input name, will be used as a key in the form submission object.
 * @property { string }     type The [html input type]{@link https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#%3Cinput%3E_types}, used to cast the input value in the form submission object.
 * @property { boolean }    [optional=false] Whether this field may be ommitted from form submission. Triggers validation.
 * @property { Validator }  [validator=null] A custom value validator, defaults to `x => true` if the input is optional, `x => Boolean(x)` otherwise.
 * @property { boolean }    [eraseOnFailure=false] Whether the field should be erased when form submission fails.
 * @property { ...any }     others [Bootstrap form control props]{@link https://react-bootstrap.github.io/components/forms/#forms-controls}
 */

const defaultValidator = (validator, optional) => validator
  ? validator
  : (optional ? (() => true) : (x => Boolean(x)));

/**
 * Automatic [Bootstrap from]{@link https://react-bootstrap.github.io/components/forms/)} aggregation.
 * 
 * @memberof AutoForm
 * @param {ControlProps} props The control properties
 * @returns JSX.Element
 */
function Control({
  name,
  defaultValue,
  type,
  optional,
  validator,
  eraseOnFailure,
  className,
  ...others
}) {

  const localValidator = defaultValidator(validator, optional);
  const [state, setState] = useState({
    value: defaultValue,
    valid: localValidator(defaultValue),
    edited: false
  });

  const { register, onChange, error } = useForm();
  useEffect(() => register(name, state.value, state.valid), []);

  const localOnChange = event => {
    const value = cast(event.target.value, type);
    const valid = localValidator(value);
    setState({ value, valid, edited: Boolean(value) });
    // TODO - Debounce
    onChange(name, value, valid);
  };

  useEffect(() => {
    if (error && eraseOnFailure) {
      const valid = localValidator('');
      setState({ value: '', valid, edited: false });
      onChange(name, '', valid);
    }
  }, [error]);

  let css = '';
  let validationState = {};
  if (state.edited) {
    css = clsx(className, state.edited && !state.valid && 'is-invalid');
    validationState = { isValid: state.valid };
  }

  return (
    <Form.Control
      {...others}
      required={!optional}
      className={css}
      onChange={localOnChange}
      type={type}
      value={state.value}
      {...validationState}
    />
  );

}

Control.propTypes = {
  name: string.isRequired,
  type: string
};

Control.defaultProps = {
  eraseOnFailure: false,
  optional: false,
  type: 'text',
  defaultValue: ''
}

Control.defaultValidator = defaultValidator;

export default Control;