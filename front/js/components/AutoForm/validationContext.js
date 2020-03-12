import { createContext, useContext } from 'react';


const Validation = createContext();


export const ValidationProvider = Validation.Provider;
export const useValidation = () => useContext(Validation);
