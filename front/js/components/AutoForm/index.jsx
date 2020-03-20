import clsx from 'clsx';
import { bool, func, number } from 'prop-types';
import React, { useEffect, useState } from 'react';
import Form from 'react-bootstrap/Form';
import Control from './Control';
import Submit from './Submit';
import Switch from './Switch';
import { FormProvider, useForm } from './formContext';
import { trace } from '../../lib';


// --------------------------------- TYPEDEFS ---------------------------------

/**
 * Function called on [AutoForm]{@link AutoForm} submission.
 * 
 * @callback submitCallback
 * @param {Object} submission The form inputs values, as a key/value map
 * @return {Promise<any>}
 * @memberof AutoForm
 */

/**
 * [AutoForm]{@link AutoForm} properties shape.
 * @typedef { Object } AutoFormProps
 * 
 * @property { submitCallback } onSubmit Callback to call on form submission
 * @property { boolean }        failure Whether a previous submission failed,
 *                                      triggers [Autoform.Control]{@link Autoform.Control} erasure
 * @property { ...any }         others [Bootstrap form props]{@link https://react-bootstrap.github.io/components/forms/}
 * @memberof AutoForm
 */

// ----------------------------------------------------------------------------

/* Not exposed! */
function InnerForm(props) {

  const { submit, validity, error } = useForm();
  /* istanbul ignore next */
  const cls = clsx(error && 'submit-failure');

  return (
    <Form
      {...props}
      noValidate
      validated={validity}
      onSubmit={submit}
      className={cls}
    />
  );

}

/**
 * @namespace
 * @param {AutoForm.AutoFormProps} props The form properties.
 * @returns { JSX.Element } The component
 * 
 * @example 
 * // A component that only accept string input with a "foo" substring and numbers between 23 and 42.
 * 
 * function MyComponent(props) {
 *   const [failure, setFailure] = useState(false);
 *   
 *   // Randomly succeed or fail on submit
 *   const onSubmit = obj => new Promise((resolve, reject) => {
 *     // Will output something like { foo: "a string with foo", bar: 37 }
 *     console.log(obj);
 *     if (Math.random() < 0.5) {
 *       setFailure(true);
 *       return reject(obj);
 *     }
 *     setFailure(false);
 *     return resolve(obj);
 *   });
 *   
 *   // Check that "foo" is in the string
 *   const fooValidator = s => s.indexOf("foo") > -1;
 * 
 *   // Check that "bar" is higher than 23 but lower than 42
 *   const barValidator = n => n > 23 && n < 42;
 * 
 *   return (
 *     <AutoForm onSubmit={onSubmit} failure={failure}>
 *       <AutoForm.Control name="foo" type="text" validator={fooValidator} />
 *       <AutoForm.Control name="bar" type="number" validator={barValidator} />
 *       <AutoForm.Submit>Submit</AutoForm.Submit>
 *     </AutoForm>
 *   );
 * }
 * 
 */
function AutoForm({ onSubmit, ...others }) {

  return (
    <FormProvider onSubmit={onSubmit}>
      <InnerForm {...others} />
    </FormProvider>
  );

}

AutoForm.propTypes = {
  onSubmit: func.isRequired,
  failureTimeout: number
};

AutoForm.Switch = Switch;
AutoForm.Control = Control;
AutoForm.Submit = Submit;
AutoForm.useForm = useForm;

export default AutoForm;
