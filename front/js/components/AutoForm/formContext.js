import React, { createContext, useContext, useState, useEffect } from 'react';


const FormContext = createContext();

export function FormProvider({ onSubmit, children }) {

  const [data, setData] = useState({});
  const [validity, setValidity] = useState(false);
  const [error, setError] = useState(false);
  const [send, setSend] = useState(null);

  const assertValidity = () => {
    setValidity(Object.keys(data).reduce(
      (acc, key) => acc && data[key].valid,
      true
    ));
  }

  const register = (name, value, optional) => {
    if (!data[name]) {
      data[name] = { name, value, valid: optional };
      assertValidity();
    }
  }

  const onChange = (name, value, valid) => {
    if (data[name].value !== value) {
      data[name] = { name, value, valid };
      assertValidity();
    }
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
    setSend(function () {
      try {
        let submission = onSubmit(result);
        return typeof submission !== Promise
          ? Promise.resolve(submission)
          : submission;
      } catch (error) {
        return Promise.reject(error);
      }
    });
  }

  // Effect-ful part of submit
  useEffect(() => {
    if (!send)
      return;

    let isSubscribed = true;
    send.catch(error => {
      if (isSubscribed)
        setError(error);
    });
    return () => isSubscribed = false;
  }, [send]);

  return (
    <FormContext.Provider value={{ register, onChange, submit, validity, error }}>
      {children}
    </FormContext.Provider>
  );

}

export const useForm = () => useContext(FormContext);
