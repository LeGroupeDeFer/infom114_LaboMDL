import React, { useRef, useState, useEffect, lazy, Suspense } from 'react';
import { Route, Switch, useLocation } from 'react-router-dom';
import {
  faUserCircle,
  faCogs,
  faInfoCircle,
  faStream,
  faBell,
  faPencilAlt,
} from '@fortawesome/free-solid-svg-icons';
import Sidebar from './Sidebar';
import layout from '../lib/layout';
import { useAuth } from '../context/authContext';

const Stream = lazy(() => import('../pages/Stream/index'));
const Detail = lazy(() => import('../pages/Stream/Detail'));
const Profile = lazy(() => import('../pages/Profile'));
const Settings = lazy(() => import('../pages/Settings'));
const About = lazy(() => import('../pages/About'));
const Notifications = lazy(() => import('../pages/Notifications'));
const Login = lazy(() => import('../pages/Login'));
const Logout = lazy(() => import('../pages/Logout'));
const Register = lazy(() => import('../pages/Register'));
const Activate = lazy(() => import('../pages/Activate'));
const Admin = lazy(() => import('../pages/Admin'));
const Recover = lazy(() => import('../pages/Recover'));
const Restore = lazy(() => import('../pages/Restore'));

// Content :: Object => Component
const Content = (_) => {
  const location = useLocation();
  const { user } = useAuth();

  const layoutStyle = layout.layout(`/${location.pathname.split('/')[1]}`);
  const links = layout.links(user);

  return (
    <>
      <Sidebar links={links} />

      <div className={`offset ${layoutStyle}`}>
        <main role="main">
          <div className="content">
            <Suspense fallback={<h1>Loading...</h1>}>
              <Switch>
                <Route path="/profile">
                  <Profile />
                </Route>

                <Route path="/notifications">
                  <Notifications />
                </Route>

                <Route path="/settings">
                  <Settings />
                </Route>

                <Route path="/about">
                  <About />
                </Route>

                <Route path="/login">
                  <Login />
                </Route>

                <Route path="/logout">
                  <Logout />
                </Route>

                <Route path="/register">
                  <Register />
                </Route>

                <Route path="/activate/:id?/:token?">
                  <Activate />
                </Route>

                <Route path="/admin">
                  <Admin />
                </Route>

                <Route path="/restore">
                  <Restore />
                </Route>

                <Route path="/recover/:id?/:token?">
                  <Recover />
                </Route>

                <Route path="/">
                  <Stream />
                </Route>
                {/* 
                <Route path="/post/:id">
                  <Detail />
                </Route> */}
              </Switch>
            </Suspense>
          </div>
        </main>
      </div>
    </>
  );
};

Content.defaultProps = {
  links: [],
};

export default Content;
