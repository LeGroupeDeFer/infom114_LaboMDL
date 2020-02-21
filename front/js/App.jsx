import React, { useState, useRef, useEffect } from 'react';
import { BrowserRouter as Router } from 'react-router-dom';
import { faUserCircle, faSlidersH, faInfoCircle, faSwimmer, faBell } from '@fortawesome/free-solid-svg-icons';
import { Content, Sidebar } from './layout';
import { scrollbarWidth, breakpoints } from './utils';
import { useWindowResize } from './hooks';

export default function App(props) {

  const [sidebarOpen, setSidebarOpen] = useState(false);
  const toggleSidebar = _ => setSidebarOpen(!sidebarOpen);
  const { width, height } = useWindowResize();
  const sidebarRef = useRef(null);
  const [dim, setDim] = useState({ width: '100%', height: '100%' });

  useEffect(() => {
    const contentWidth = width < breakpoints['md']
      ? '100%'
      : (width - scrollbarWidth() - (
        sidebarRef.current ? sidebarRef.current.offsetWidth : 0
      )) + 'px';
    const contentHeight = width < breakpoints['md']
      ? (height - sidebarRef.current ? sidebarRef.current.offsetWidth : 0) + 'px'
      : '100%';
    setDim({ width: contentWidth, height: contentHeight });
  }, [sidebarRef, width, height]);

  /* TODO - User handling */
  const user = {
    firstname: 'John',
    lastname: 'Doe',
    picture: 'https://via.placeholder.com/240'
  };

  const links = [
    { name: 'stream', path: '/', icon: faSwimmer },
    { name: 'profile', path: '/profile', icon: faUserCircle },
    { name: 'notifications', path: '/notifications', icon: faBell },
    { name: 'settings', path: '/settings', icon: faSlidersH },
    { name: 'about', path: '/about', icon: faInfoCircle }
  ];

  return (
    <React.StrictMode>
      <Router>
        <Sidebar
          ref={sidebarRef}
          open={sidebarOpen}
          onClick={toggleSidebar}
          user={user}
          links={links}
        />
        <Content
          className='p-3'
          style={{ width: dim.width, height: dim.height }}
        />
      </Router>
    </React.StrictMode>
  );

}
