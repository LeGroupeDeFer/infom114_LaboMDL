import React, { useEffect, useState } from 'react';

import Container from 'react-bootstrap/Container';
import ButtonGroup from 'react-bootstrap/ButtonGroup';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import OverlayTrigger from 'react-bootstrap/OverlayTrigger';
import Tooltip from 'react-bootstrap/Tooltip';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import 'regenerator-runtime';

import StreamContent from '../pages/Stream/index';
import { useAuth } from '../context';
import { Authenticated } from '../components/';
import Moment from '../components/Moment';

function Profile() {
  const { user } = useAuth();

  return (
    <>
      <Container fluid className="menu-bar-container py-2">
        <Menu className="menu-bar" />
      </Container>

      <Container className="mt-5 py-5">
        <Row>
          <Col xs={12} md={5}>
            <div className="fixed-info">
              <h1 className="p-3 text-dark">
                <h3><b>{user.firstname} {user.lastname}</b></h3>
              </h1>
              {user ? <User user={user} /> : <></>}
            </div>
          </Col>
          <Col xs={12} md={7}>
            <h2 className="text-dark px-3">
              <b>Vos publications</b>
              <hr />
            </h2>
            {user ? <StreamContent userId={user.id} /> : <></>}
          </Col>
        </Row>
      </Container>
    </>
  );
}

const Menu = () => {
  return (
    <Row>
      <Col xs={5} md={5}></Col>
      <Col md={1} xs={1}>
        <ButtonGroup className="kind-section d-flex justify-content-between">
          <OverlayTrigger
            key={1}
            placement="bottom"
            overlay={<Tooltip>Se déconnecter</Tooltip>}
          >
            <a key={1} href="/logout" className={'kind-choice'}>
              <Icon icon="door-open" />
            </a>
          </OverlayTrigger>
        </ButtonGroup>
      </Col>
      <Col xs={5} md={5}></Col>
    </Row>
  );
};

const User = ({ user }) => {
  return (
    <Row>
      <Col>
        <hr />
        <p><b className="text-secondary">Email :</b> {user.email}</p>
        <p><b className="text-secondary">A rejoint le :</b> <Moment date={user.creationDate} /></p>
        <hr />
      </Col>
    </Row>
  );
};

const AuthenticatedProfile = Authenticated(Profile);
export default AuthenticatedProfile;
