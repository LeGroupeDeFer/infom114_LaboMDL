import React, { useEffect, useState } from 'react';
import { string, arrayOf, shape } from 'prop-types';
import ReSelect from 'react-select';
import { useForm } from './formContext';
import clsx from 'clsx';

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

const defaultValidator = (validator, optional, isMulti) => validator
  ? validator
  : (optional ? (() => true) : (x => (isMulti ? x.length > 0 : Boolean(x))));

/**
 * Automatic [Bootstrap from]{@link https://react-bootstrap.github.io/components/forms/)} aggregation.
 *
 * @memberof AutoForm
 * @param {ControlProps} props The control properties
 * @returns JSX.Element
 */
function Select({
 name,
 defaultValue,
 optional,
 validator,
 eraseOnFailure,
 className,
 options,
 isMulti,
 ...others
}) {

  const localValidator = defaultValidator(validator, optional, isMulti || false);
  const [state, setState] = useState({
    value: defaultValue,
    valid: localValidator(defaultValue),
    edited: false
  });
  const resetValue = isMulti ? [] : null;
  const [localValue, setLocalValue] = useState(null);

  const { register, onChange, error } = useForm();
  useEffect(() => register(name, state.value, state.valid), []);

  const localOnChange = (value, action) => {
    const listedValue = value || (isMulti ? [] : null);
    const valid = localValidator(listedValue);
    setLocalValue(listedValue);
    const liftedValue = (isMulti ? listedValue.map(v => v.value) : listedValue.value);
    setState({ value: liftedValue, valid, edited: Boolean(value) });
    // TODO - Debounce
    onChange(name, liftedValue, valid);
  };

  useEffect(() => {
    if (error && eraseOnFailure) {
      const valid = localValidator(isMulti ? [] : null);
      setState({ value: '', valid, edited: false });
      setLocalValue(null);
      onChange(name, null, valid);
    }
  }, [error]);

  let css = '';
  let validationState = {};
  if (state.edited) {
    css = clsx(className, state.edited && !state.valid && 'is-invalid');
    validationState = { isValid: state.valid };
  }

  return (
    <ReSelect
      {...others}
      isMulti={isMulti}
      required={!optional}
      className={css}
      onChange={localOnChange}
      value={localValue}
      options={options}
      {...validationState}
    />
  );

}

Select.propTypes = {
  name: string.isRequired,
  options: arrayOf(shape({
    label: string.isRequired,
    value: string.isRequired
  }))
};

Select.defaultProps = {
  eraseOnFailure: false,
  optional: false,
  defaultValue: '',
  isMulti: false,
  options: [],
}

Select.defaultValidator = defaultValidator;

export default Select;