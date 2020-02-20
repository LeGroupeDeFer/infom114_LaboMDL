import React, { lazy, Suspense } from 'react';
import Spinner from 'react-bootstrap/Spinner';
import { Route, Switch } from 'react-router-dom';
import clsx from 'clsx';

const CenteredSpinner = _ => (
  <Spinner
    animation='grow'
    role='status'
    className='abs-center'
  >
    <span className="sr-only">Loading...</span>
  </Spinner>
);

const Waiting = Component => props => (
  <Suspense fallback={<CenteredSpinner />}>
    <Component {...props} />
  </Suspense>
);

const Stream = lazy(() => new Promise(resolve =>
  setTimeout(() => resolve(import('../pages/Stream')), 3000)
));
const Profile = lazy(() => new Promise(resolve =>
  setTimeout(() => resolve(import('../pages/Profile')), 3000)
));
const Settings = lazy(() => new Promise(resolve =>
  setTimeout(() => resolve(import('../pages/Settings')), 3000)
));
const About = lazy(() => new Promise(resolve =>
  setTimeout(() => resolve(import('../pages/About')), 3000)
));

export default function Content(props) {
  return (
    <div className={clsx('content', props.className)}>
      <Switch>
        <Route exact path='/' component={Waiting(Stream)} />
        <Route path='/profile' component={Waiting(Profile)} />
        <Route path='/settings' component={Waiting(Settings)} />
        <Route path='/about' component={Waiting(About)} />
      </Switch>
    </div>
  );
}
