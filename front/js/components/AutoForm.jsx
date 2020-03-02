import React, { useState, createContext, useContext, useEffect } from 'react';
import { func, arrayOf, shape, string, bool, any } from 'prop-types';
import Form from 'react-bootstrap/Form';
import Button from 'react-bootstrap/Button';
import clsx from 'clsx';


const Validation = createContext();


function AutoForm({ onSubmit, error, children, ...others }) {

  const [state, setState] = useState({});
  const [validated, setValidated] = useState(false);

  // Setup value, validator and eraseOnFailure on input register
  const register = name => (value, validator, eraseOnFailure) =>
    setState(oldState => ({
      ...oldState, [name]: {
        value, validator, eraseOnFailure, isValid: validator(value)
      }
    }));

  // When an input change, update its value and its validity
  const onChange = name => value =>
    setState(oldState => ({
      ...oldState, [name]: {
        ...oldState[name], value, isValid: oldState[name].validator(value)
      }
    }));

  // Prepare the input bindings
  const binder = name => ({
    register: register(name),
    onChange: onChange(name),
    value: state[name] ? state[name].value : "",
    isValid: state[name] ? state[name].validator(state[name].value) : false
  });

  // On submit, simply prevent the default action and send the data
  const submit = event => {
    event.preventDefault();
    const submission = Object.keys(state).reduce(
      (acc, key) => ({ ...acc, [key]: state[key].value }),
      {}
    );
    onSubmit(submission);
  }

  // On error, erase inputs which subscribed for erasure
  useEffect(() => {
    if (!error)
      return;

    const erased = Object.keys(state)
      .filter(key => state[key].eraseOnFailure)
      .reduce((acc, k) => ({ ...acc, [k]: { ...state[k], value: "" } }), {});

    setState(oldState => ({ ...oldState, ...erased }))
  }, [error]);

  // The form validation is equal to its inputs validations
  useEffect(() => setValidated(Object.keys(state).reduce(
    (a, key) => a && state[key].validator(state[key].value), true
  )), [state]);

  return (
    <Validation.Provider value={{ binder, validated, error }}>
      <Form
        className={clsx(error && 'submit-failure')}
        {...others}
        noValidate
        validated={validated}
        onSubmit={submit}
      >
        {children}
      </Form>
    </Validation.Provider>
  );

}

AutoForm.propTypes = {
  onSubmit: func.isRequired,
  fields: arrayOf(shape({
    name: string.isRequired,
    value: any,
    required: bool,
    validator: func
  }))
};


function AutoFormControl({
  className,
  name,
  defaultValue,
  eraseOnFailure,
  optional,
  validator,
  ...others
}) {

  // Setup a validator if we don't have one
  const localValidator = validator
    ? validator
    : (optional ? (() => true) : (x => Boolean(x)));

  // Register the field to the validation context on first rendering
  useEffect(() => {
    register(value, localValidator, eraseOnFailure);
  }, []);

  // Get validation context values and hooks
  const { binder, error } = useContext(Validation);
  const { onChange, value, isValid, register } = binder(name);

  const localOnChange = event => onChange(event.target.value);

  const cssClass = clsx(className, !isValid && 'is-invalid');

  return (
    <Form.Control
      {...others}
      className={cssClass}
      required={!optional}
      onChange={localOnChange}
      value={value}
      isValid={isValid}
    />
  );

}

AutoFormControl.propTypes = {
  name: string.isRequired,
  value: any
};

AutoFormControl.defaultProps = {
  eraseOnFailure: false
}


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


function AsyncForm({ endpoint, andThen, ...others }) {

  const [error, setError] = useState(false);

  const onSubmit = data => api(endpoint, data)
    .then(andThen)
    .catch(e => setError(e.message));

  return (
    <AutoForm
      error={error}
      onSubmit={onSubmit}
      {...others}
    />
  );

}


AutoForm.Control = AutoFormControl;
AutoForm.Submit = AutoFormSubmit;
AutoForm.Async = AsyncForm;

export default AutoForm;
