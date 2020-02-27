import React, { createContext, useContext } from 'react';
import { useAuth } from './authContext';


const UserContext = createContext();

function UserProvider(props) {
  const { data: { user } } = useAuth();

  return <UserContext.Provider value={user} {...props} />;
}

function useUser() {
  const context = useContext(UserContext);
  if (context === undefined) {
    throw new Error('useUser must be used within a UserProvider');
  }
  return context;
}


export default { UserProvider, useUser };