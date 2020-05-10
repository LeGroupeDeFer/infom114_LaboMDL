import React, { useEffect, useState } from 'react';
import { NavLink } from 'react-router-dom';

import ListGroup from 'react-bootstrap/ListGroup';
import Container from 'react-bootstrap/Container';
import Button from 'react-bootstrap/Button';
import ButtonGroup from 'react-bootstrap/ButtonGroup';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import Card from 'react-bootstrap/Card';
import OverlayTrigger from 'react-bootstrap/OverlayTrigger';
import Tooltip from 'react-bootstrap/Tooltip';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import clsx from 'clsx';

import { useAuth } from '../context';
import { Authenticated } from '../components/';
import api from '../lib/api';
import Post from 'unanimity/components/Post';

function Profile() {

  const { user } = useAuth();
  const [posts, setPosts] = useState([]);

  useEffect(() => {

    const fetch = async () => {
      let res = await api.users.posts(user.id);
      console.log(res);
      setPosts(res);
    };
    fetch();

  }, []);



  return (
    <>
      <Menu />
      <Container>
        <h2 className="mb-3 mt-3">
          <span className="mr-3"><Icon icon="users" /> <b>Profil utilisateur</b></span>
        </h2>
        <hr />
        <Row>
          <Col style={{ 'border-right': '1px dashed #333' }} xs={12} md={4}>
            <User user={user} />
          </Col>
          <Col xs={12} md={8}>

          {
            posts.map(post => {
              return (<><Post post={post}/><hr /></>);
            })
          }

          </Col>
        </Row>
      </Container>
    </>
  );
};


const Menu = () => {

  return (
    <Row>
      <Col xs={2}></Col>
      <Col xs={8}>
        <ButtonGroup className="kind-section d-flex justify-content-between">
          <OverlayTrigger
            placement="bottom"
            overlay={<Tooltip>Se déconnecter</Tooltip>}
          >
            <a href="/logout" >
              <Icon icon="door-open" />
              <span> - Se déconnecter</span>
            </a>
          </OverlayTrigger>
        </ButtonGroup>
      </Col>
      <Col xs={2}></Col>
    </Row>
  );
};

const User = ({ user }) => {

  return (
    <Row>
      <Col>
        <div>
          <img
            src="https://www.freelogodesign.org/file/app/client/thumb/93b27fc8-6653-43ea-a4c4-3f22893f93dd_200x200.png?1585111496240"
            alt="logo"
            maxHeight="auto"
            maxWidth="100%"
          />
        </div>
      </ Col>
      <Col>
        <span><b>{user.firstname}</b> </span>
        <span><b>{user.lastname}</b> </span>
        <hr />
        <span>Email : {user.email}</span><br />
        <span>A rejoit le : {user.creationDate.substring(0, 10)}</span>
      </ Col>
    </Row>
  );
};

const AuthenticatedProfile = Authenticated(Profile);
export default AuthenticatedProfile;


/*

              onDelete={onDelete}
              onFlag={onFlag}
              onFlagCancel={onFlagCancel}
              onHide={onHide}
              onVote={(vote) => onVote(post, vote)}
              onPreview={onPreview}
              onTag={onTag}
*/