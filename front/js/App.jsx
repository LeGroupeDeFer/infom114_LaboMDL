import React, { useState, useRef } from 'react';
import { BrowserRouter as Router } from 'react-router-dom';
import { Content, Sidebar } from './layout';
import {
  faUserCircle, faSlidersH, faInfoCircle, faSwimmer, faBell
} from '@fortawesome/free-solid-svg-icons';


/* Fake user */
const user = {
  firstname: 'John',
  lastname: 'Doe',
  picture: 'https://via.placeholder.com/240'
};

/* Module path should be put here instead of within the Content */
const links = [
  { name: 'stream', path: '/', icon: faSwimmer },
  { name: 'profile', path: '/profile', icon: faUserCircle },
  { name: 'notifications', path: '/notifications', icon: faBell },
  { name: 'settings', path: '/settings', icon: faSlidersH },
  { name: 'about', path: '/about', icon: faInfoCircle }
];


// App :: None => Component
function App(_) {

  return (
    <React.StrictMode>
      <Router>
        <Content links={links} />
      </Router>
    </React.StrictMode>
  );

}


export default App;
