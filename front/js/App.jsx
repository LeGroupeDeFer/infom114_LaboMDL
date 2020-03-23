import React, { useState, useRef, lazy, Suspense } from 'react';
import Spinner from 'react-bootstrap/Spinner';
import { BrowserRouter as Router } from 'react-router-dom';
import { AuthProvider } from './context/authContext';


const Loading = () => (
  <Spinner
    animation='grow'
    role='status'
    className='abs-center text-primary'
  >
    <span className="sr-only">Loading...</span>
  </Spinner>
);

const Matter = lazy(() => import('./layout/Content'));


// App :: None => Component
function App(_) {

  return (
    <React.StrictMode>
      <AuthProvider>
        <Suspense fallback={<Loading />}>
          <Router>
            <Matter />
          </Router>
        </Suspense>
      </AuthProvider>
    </React.StrictMode>
  );

}


export default App;
