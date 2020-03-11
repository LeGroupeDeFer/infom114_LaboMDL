import React, { useState, createContext, useContext, useEffect } from 'react';
import { func, arrayOf, shape, string, bool, any } from 'prop-types';
import Form from 'react-bootstrap/Form';
import Button from 'react-bootstrap/Button';
import clsx from 'clsx';

const Validation = createContext();


function AutoForm({ onSubmit, failure, ...others }) {

  const [state, setState] = useState({});
  const [validated, setValidated] = useState(false);

  const onInputChange = name => ({ value, isValid }) => {
    setState(oldState => ({ ...oldState, [name]: { value, isValid } }));
  };

  const onFormSubmit = event => {
    event.preventDefault();
    onSubmit(Object.keys(state).reduce(
      (acc, key) => ({ ...acc, [key]: state[key].value }), {}
    ));
  };

  const binding = name => onInputChange(name);

  useEffect(() => setValidated(Object.keys(state).reduce(
    (acc, key) => acc && state[key].isValid, true
  )), [state]);

  return (
    <Validation.Provider value={{ binding, validated, failure }}>
      <Form
        {...others}
        noValidate
        validated={validated}
        onSubmit={onFormSubmit}
        className={clsx(failure && 'submit-failure')}
      />
    </Validation.Provider>
  );

}

AutoForm.propTypes = {
  onSubmit: func.isRequired,
  failure: bool.isRequired
};


const defaultValidator = (validator, optional) => validator
  ? validator
  : (optional ? (() => true) : (x => Boolean(x)));

function AutoFormControl({
  name,
  type,
  optional,
  validator,
  eraseOnFailure,
  className,
  ...others
}) {

  const localValidator = defaultValidator(validator, optional);

  const { binding, failure } = useContext(Validation);
  const onChange = binding(name);
  useEffect(() => onChange({ value: '', isValid: optional }), []);

  const [state, setState] = useState({
    value: '', isValid: optional, edited: false
  });

  const localOnChange = event => {
    const value = type === 'number'
      ? Number(event.target.value)
      : event.target.value;
    const isValid = localValidator(value);
    setState({ value, isValid, edited: Boolean(value) });
    onChange({ value, isValid });
  };

  useEffect(() => {
    if (failure && eraseOnFailure)
      setState({ value: '', isValid: false, edited: false });
  }, [failure]);

  let css = '';
  let validationState = {};
  if (state.edited) {
    css = clsx(className, state.edited && !state.isValid && 'is-invalid');
    validationState = { isValid: state.isValid };
  }

  return (
    <Form.Control
      {...others}
      required={!optional}
      className={css}
      onChange={localOnChange}
      type={type}
      value={state.value || ''}
      {...validationState}
    />
  );

}

AutoFormControl.propTypes = {
  name: string.isRequired,
  type: string
};

AutoFormControl.defaultProps = {
  eraseOnFailure: false,
  type: 'text'
}


function AutoFormSwitch({ optional, name, className, variant, ...others }) {

  const [toggled, setToggled] = useState(false);
  const localValidator = value => optional ? true : value;
  const { binding, failure } = useContext(Validation);
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

AutoFormSwitch.defaultProps = {
  variant: 'primary'
};

AutoFormSwitch.propTypes = {
  name: string.isRequired,
  optional: bool,
  className: string,
  variant: string,
};


function AutoFormSubmit({ className, ...others }) {
  const { validated, error } = useContext(Validation);
  return (
    <Button
      type="submit"
      disabled={!validated}
      className={`${className} ${error ? 'btn-danger' : ''}`}
      {...others}
    />
  );
}


AutoForm.Control = AutoFormControl;
AutoForm.Switch = AutoFormSwitch;
AutoForm.Submit = AutoFormSubmit;


export default AutoForm;
