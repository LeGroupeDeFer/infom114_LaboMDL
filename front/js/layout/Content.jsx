import React, { useRef, useState, useEffect, lazy, Suspense } from 'react';
import { Route, Switch, useLocation } from 'react-router-dom';
import {
  faUserCircle,
  faCogs,
  faInfoCircle,
  faStream,
  faBell,
  faPencilAlt
} from '@fortawesome/free-solid-svg-icons';
import Sidebar from './Sidebar';
import { useAuth } from '../context/authContext';

const Stream = lazy(() => import('../pages/Stream'));
const Profile = lazy(() => import('../pages/Profile'));
const Settings = lazy(() => import('../pages/Settings'));
const About = lazy(() => import('../pages/About'));
const Notifications = lazy(() => import('../pages/Notifications'));
const Login = lazy(() => import('../pages/Login'));
const Logout = lazy(() => import('../pages/Logout'));
const Register = lazy(() => import('../pages/Register'));
const Create = lazy(() => import('../pages/Create'));
const Activate = lazy(() => import('../pages/Activate'));

// Content :: Object => Component
const Content = _ => {
  const location = useLocation();
  const { user } = useAuth();
  const locationClass = location.pathname.split('/')[1];

  const links = [
    { name: 'stream', path: '/', icon: faStream, title: 'Fil d"actualité' },
    { name: 'about', path: '/about', icon: faInfoCircle, title: 'À propos' }
  ];
  if (user)
    links.push(
      {
        name: 'profile',
        path: '/profile',
        icon: faUserCircle,
        title: 'Profil'
      },
      {
        name: 'notifications',
        path: '/notifications',
        icon: faBell,
        title: 'Notifications'
      }
    );
  links.push({
    name: 'settings',
    path: '/settings',
    icon: faCogs,
    title: 'Paramètres'
  });

  return (
    <>
      <Sidebar links={links} />

      <div className={`offset ${locationClass}`}>
        <main role="main">
          <div className="content">
            <Suspense fallback={<h1>Loading...</h1>}>
              <Switch>
                <Route exact path="/">
                  <Stream />
                </Route>

                <Route path="/create">
                  <Create />
                </Route>

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
              </Switch>
            </Suspense>
          </div>
        </main>
      </div>
    </>
  );
};

Content.defaultProps = {
  links: []
};

export default Content;
