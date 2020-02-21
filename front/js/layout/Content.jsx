import React, { lazy, forwardRef } from 'react';
import { Route, Switch } from 'react-router-dom';
import Waiting from '../components/Waiting';
import { fakeLatency } from '../utils';
import clsx from 'clsx';


const Stream = lazy(() => new Promise(resolve =>
  setTimeout(() => resolve(import('../pages/Stream')), fakeLatency)
));
const Profile = lazy(() => new Promise(resolve =>
  setTimeout(() => resolve(import('../pages/Profile')), fakeLatency)
));
const Settings = lazy(() => new Promise(resolve =>
  setTimeout(() => resolve(import('../pages/Settings')), fakeLatency)
));
const About = lazy(() => new Promise(resolve =>
  setTimeout(() => resolve(import('../pages/About')), fakeLatency)
));

const Notifications = lazy(() => new Promise(resolve =>
  setTimeout(() => resolve(import('../pages/Notifications')), fakeLatency)
));

const Content = forwardRef((props, ref) => {
  const { className, ...otherProps } = props;
  return (
    <div
      ref={ref}
      className={clsx('content', className)}
      {...otherProps}
    >
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

export default Content;