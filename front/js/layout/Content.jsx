import 'unanimity/lib/icons'; // Preload the icon lib

import React, { lazy, Suspense } from 'react';
import { Route, Switch, useLocation } from 'react-router-dom';
import { Loading } from 'unanimity/components';
import Sidebar from './Sidebar';
import layout from '../lib/layout';
import { StreamProvider, useAuth } from '../context';

const Stream = lazy(() => import('../pages/Stream/index'));
const Profile = lazy(() => import('../pages/Profile'));
const Settings = lazy(() => import('../pages/Settings'));
const About = lazy(() => import('../pages/About'));
const Notifications = lazy(() => import('../pages/Notifications'));
const Login = lazy(() => import('../pages/Login'));
const Logout = lazy(() => import('../pages/Logout'));
const Register = lazy(() => import('../pages/Register'));
const Activate = lazy(() => import('../pages/Activate'));
const AuthenticatedAdmin = lazy(() => import('../pages/Admin'));
const Recover = lazy(() => import('../pages/Recover'));
const Restore = lazy(() => import('../pages/Restore'));

const Content = (_) => {
  const location = useLocation();
  const { user, token } = useAuth();

  const layoutStyle = layout.layout(`/${location.pathname.split('/')[1]}`);
  const links = layout.links(user, token);

  return (
    <>
      <Sidebar links={links} />

      <div className={`offset ${layoutStyle}`}>
        <main role="main">
          <StreamProvider>
            <div className="content">
              <Suspense fallback={<Loading />}>
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
                    <AuthenticatedAdmin />
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
                </Switch>
              </Suspense>
            </div>
          </StreamProvider>
        </main>
      </div>
    </>
  );
};

Content.defaultProps = {
  links: [],
};

export default Content;
