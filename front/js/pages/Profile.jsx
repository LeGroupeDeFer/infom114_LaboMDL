import React, { useEffect, useState } from 'react';

import Container from 'react-bootstrap/Container';
import ButtonGroup from 'react-bootstrap/ButtonGroup';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import OverlayTrigger from 'react-bootstrap/OverlayTrigger';
import Tooltip from 'react-bootstrap/Tooltip';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import 'regenerator-runtime';

import Stream from '../pages/Stream/index';
import { useAuth } from '../context';
import { Authenticated } from '../components/';
import api from '../lib/api';
import Moment from '../components/Moment';

function Profile() {
  const { user } = useAuth();
  const [posts, setPosts] = useState();

  const fetch = async () => {
    let res = await api.users.posts(user.id);
    setPosts(res);
  };

  useEffect(() => {
    fetch();
  }, []);

  return (
    <>
      <Container fluid className="menu-bar-container py-2">
        <Menu className="menu-bar" />
      </Container>
      <br />
      <br />
      <br />
      <Container>
        <Row>
          <Col xs={12} md={5}>
            <div className="fixed-info">
              <h2 className="mb-3 mt-3">
                <span className=" mr-3">
                  <Icon icon="user" /> <b>Profil utilisateur</b>
                </span>
              </h2>
              {user ? <User user={user} /> : <></>}
            </div>
          </Col>
          <Col xs={12} md={7}>
            {posts ? <Stream onlySpecificPosts={posts} /> : <></>}
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
      <Col xs={6} md={12}>
        <div>
          <img
            src="https://www.freelogodesign.org/file/app/client/thumb/93b27fc8-6653-43ea-a4c4-3f22893f93dd_200x200.png?1585111496240"
            alt="logo"
            className="profile-pic"
          />
        </div>
      </Col>
      <Col xs={6} md={7}>
        <hr />
        <span>
          <b>{user.firstname}</b>{' '}
        </span>
        <span>
          <b>{user.lastname}</b>{' '}
        </span>
        <hr />
        <span>Email : {user.email}</span>
        <br />
        <span>
          A rejoint le : <Moment date={user.creationDate} />
        </span>
        <hr />
      </Col>
    </Row>
  );
};

const AuthenticatedProfile = Authenticated(Profile);
export default AuthenticatedProfile;
