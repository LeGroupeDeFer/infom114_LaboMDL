import React, {useEffect} from 'react';
import { useAuth } from '../context';
import { Container, Row, Col } from 'react-bootstrap';
import { useHistory, Link } from 'react-router-dom';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import {trace} from "../lib";

export function Authenticated(Component) {
  return function(props) {
    const { user } = useAuth();
    const history = useHistory();

    useEffect(() => user ? undefined : history.replace('/login'), []);
    if (!user)
      return <></>;

    return <Component {...props} />;
  };
}

export function Unauthenticated(Component) {
  return function(props) {
    const { user } = useAuth();
    const history = useHistory();

    useEffect(() => { if (!!user) history.replace('/') }, []);
    if (user)
      return <></>;

    return <Component {...props} />;
  };
}

export const WhenLogged = (Component) => (props) => {
  const isLogged = !!useAuth().user;
  return isLogged ? <Component {...props} /> : null;
};

export const AuthDisabled = (Component) => (props) => {
  const isLogged = !!useAuth().user;
  return <Component disabled={!isLogged} {...props} />;
};

export const May = (cap, Component, ErrorComponent = null) => (props) => {
  const { token } = useAuth();
  if (token && token.cap.some((e) => e.name === cap))
    return <Component {...props} />;
  return ErrorComponent ? <ErrorComponent cap={cap} {...props} /> : <></>;
};

export const Dialog = Unauthenticated(({ icon, title, children }) => {
  return (
    <Container className="recovery-form text-center">
      <Row>
        <Col className="text-secondary">
          <h1>
            <Icon icon={icon} size="1x" className="mr-3" />
            <b>{title}</b>
          </h1>
          <hr />
        </Col>
      </Row>

      <Row>
        <Col md={{ span: 6, offset: 3 }}>{children}</Col>
      </Row>

      <Row>
        <Col>
          <hr />
          <Link to="/" className="text-secondary mr-2">
            <b>Home</b>
          </Link>
          <Link to="/login" className="text-secondary mr-2">
            <b>Login</b>
          </Link>
          <Link to="/about" className="text-secondary">
            <b>FAQ</b>
          </Link>
        </Col>
      </Row>
    </Container>
  );
});
