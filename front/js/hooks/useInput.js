
import { useState } from "react";

function useInput(initialValue, validator) {
  validator = validator || (x => x);
  const [value, setValue] = useState(initialValue);
  const [valid, setValid] = useState(true);
  const onChange = event => {
    setValue(event.target.value);
    setValid(Boolean(validator(event.target.value)));
  };

  return {
    value,
    setValue,
    valid,
    reset: () => setValue(""),
    bind: {
      value,
      isValid: valid,
      onChange: event => {
        setValue(event.target.value);
      }
    }
  };
}

export default useInput;