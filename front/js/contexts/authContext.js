import React, { createContext, useContext } from 'react';


const AuthContext = createContext();

function AuthProvider(props) {
  
}

const useAuth = () => useContext(AuthContext);


export default { AuthProvider, useAuth };