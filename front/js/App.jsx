import React, { useState } from 'react';
import { BrowserRouter as Router } from 'react-router-dom';
import { faUserCircle, faSlidersH, faInfoCircle, faSwimmer } from '@fortawesome/free-solid-svg-icons';
import { Content, Sidebar } from './layout';

export default function App(props) {

  const [sidebarOpen, setSidebarOpen] = useState(false);
  const toggleSidebar = _ => setSidebarOpen(!sidebarOpen);

  /* TODO - User handling */
  const user = {
    firstname: 'John',
    lastname: 'Doe',
    picture: 'https://via.placeholder.com/240'
  };

  const links = [
    { name: 'stream', path: '/', icon: faSwimmer },
    { name: 'profile', path: '/profile', icon: faUserCircle },
    { name: 'settings', path: '/settings', icon: faSlidersH },
    { name: 'about', path: '/about', icon: faInfoCircle }
  ];

  return (
    <React.StrictMode>
      <Router>
        <Sidebar
          open={sidebarOpen}
          onClick={toggleSidebar}
          user={user}
          links={links}
        />
        <Content className='p-2' />
      </Router>
    </React.StrictMode>
  );

}
