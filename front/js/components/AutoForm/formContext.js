import React, { createContext, useContext, useState, useEffect } from 'react';


const FormContext = createContext();

const assertValidity = data =>
  Object.keys(data).reduce((acc, key) => acc && data[key].valid, true);

export function FormProvider({ onSubmit, validator, children }) {

  const [data, setData] = useState({});
  const [validity, setValidity] = useState(false);
  const [send, setSend] = useState(null);

  // TODO - Test double registers
  const register = (name, value, valid) => {
    if (!data[name])
      setData(data => ({ ...data, [name]: { name, value, valid } }));
  }

  // TODO - Test Useless onChange
  const onChange = (name, value, valid) => {
    if (data[name].value !== value)
      setData(data => ({ ...data, [name]: { name, value, valid } }));
  }

  const submit = event => {
    if (event)
      event.preventDefault();
    if (!validity)
      return onSubmit(Error('Invalid form.'));

    const result = Object.keys(data).reduce(
      (acc, key) => ({ ...acc, [key]: data[key].value }),
      {}
    );

    // At this point we have the data we need to send upstream. However as the
    // API imply asynchronous logic and that we currently are within rendering,
    // we can't simply start the asynchronous code; it could, down the line,
    // operate on an unmounted component. What we do isntead is that we save
    // the request function in a state variable which will be executed on the
    // next effect cycle
    setSend({
      promise: (() => {
        try {
          let submission = onSubmit(result);
          return typeof submission !== Promise
            ? Promise.resolve(submission)
            : submission;
        } catch (error) {
          return Promise.reject(error);
        }
      })()
    });
  }

  useEffect(() => setValidity(validator(data) && assertValidity(data)), [data]);

  return (
    <FormContext.Provider value={{ register, onChange, submit, validity, data }}>
      {children}
    </FormContext.Provider>
  );

}

export const useForm = () => useContext(FormContext);
