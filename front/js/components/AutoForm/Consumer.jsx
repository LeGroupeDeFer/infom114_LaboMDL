import React from 'react';
import { useForm } from './formContext';


function Consumer(Component) {
  const { data } = useForm();
  return props => <Component data={data} {...props} />;
}


export default Consumer;