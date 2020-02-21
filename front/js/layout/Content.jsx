import React, { forwardRef, useState, useEffect } from 'react';
import { Route, Switch } from 'react-router-dom';
import Waiting from '../components/Waiting';
import useWindowResize from '../hooks/useWindowResize';
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


// Content :: Object => Component
const Content = forwardRef(({ sidebar }, ref) => {

  /* Resizing logic */
  const [dim, setDim] = useState({ width: '100%', height: '100%' });
  const { width, height } = useWindowResize();

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
    <div ref={ref} className='content p-3' style={dim}>
      <Switch>
        <Route exact path='/' component={Waiting(Stream)} />
        <Route path='/profile' component={Waiting(Profile)} />
        <Route path='/notifications' component={Waiting(Notifications)} />
        <Route path='/settings' component={Waiting(Settings)} />
        <Route path='/about' component={Waiting(About)} />
      </Switch>
    </div>
  );
});

Content.defaultProps = {
  sidebar: null
};


export default Content;
