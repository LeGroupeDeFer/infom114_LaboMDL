import React, { useRef, useState, useEffect } from 'react';
import Sidebar from './Sidebar';
import Waiting from '../components/Waiting';
import useWindowResize from '../hooks/useWindowResize';
import { Route, Switch, useLocation } from 'react-router-dom';
import { breakpoints, scrollbarWidth } from '../utils';
import { delay } from '../utils/dev';

/**
 * The following "components" are delayed while the layout construction is in
 * progress!
 * 
 * @see delay
 */
const Stream = delay(() => import('unanimity/pages/Stream'));
const Profile = delay(() => import('unanimity/pages/Profile'));
const Settings = delay(() => import('unanimity/pages/Settings'));
const About = delay(() => import('unanimity/pages/About'));
const Notifications = delay(() => import('unanimity/pages/Notifications'));
const Login = delay(() => import('unanimity/pages/Login'));


// Content :: Object => Component
const Content = ({ links }) => {

  /* Resizing logic */
  const [dim, setDim] = useState({ width: '100%', height: '100%' });
  const { width, height } = useWindowResize();
  const sidebar = useRef(null);

  const location = useLocation();
  const locationClass = location.pathname.replace(/\W/g, '');

  useEffect(() => {
    const sidebarWidth = sidebar.current ? sidebar.current.offsetWidth : 0;
    const sidebarHeight = sidebar.current ? sidebar.current.offsetHeight : 0;

    const contentWidth = width < breakpoints['md']
      ? '100%' : `${width - scrollbarWidth() - sidebarWidth}px`;
    const contentHeight = width < breakpoints['md']
      ? `${(height - sidebarHeight)}px` : '100%';

    setDim({ width: contentWidth, height: contentHeight });
  }, [sidebar, width, height]);
  /* /Resizing logic */

  return (
    <>
      <Sidebar ref={sidebar} links={links} />
      <div className={`content ${locationClass} p-3`} style={dim}>
        <Switch>
          <Route exact path='/' component={Waiting(Stream)} />
          <Route path='/profile' component={Waiting(Profile)} />
          <Route path='/notifications' component={Waiting(Notifications)} />
          <Route path='/settings' component={Waiting(Settings)} />
          <Route path='/about' component={Waiting(About)} />
          <Route path='/login' component={Waiting(Login)} />
        </Switch>
      </div>
    </>
  );
};

Content.defaultProps = {
  sidebar: {}
};


export default Content;
