import React, { useEffect } from 'react';
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

function Profile() {

  return (
    <>
      <Menu />
      <Container>
        <h2 className="mb-3 mt-3">
          <span className="mr-3"><Icon icon="users" /> <b>Profil utilisateur</b></span>
        </h2>
        <hr />
        <Row>
          <Col style={{ 'border-right': '1px dashed #333'}}>
            <User />
          </Col>
          <Col >
            <Postes />
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

const User = () => {

  const {user} = useAuth();

  return (
    <span>Le profil utilisateur sera ici</span>
  );
};

const Postes = () => {

  return (
    <span>Les postes seront ici</ span>
  );
};

const AuthenticatedProfile = Authenticated(Profile);
export default AuthenticatedProfile;


/*

  const {user} = useAuth();

  useEffect( () => {
    console.log(user);
    
    const fetch = async () => {
      let posts = await api.users.posts(user.id);
      console.log(posts);
    }
    fetch();
   
  }, []);


    <Row>
    <Col>
      <div className="sidebar-logo">
        <img
          src="https://www.freelogodesign.org/file/app/client/thumb/93b27fc8-6653-43ea-a4c4-3f22893f93dd_200x200.png?1585111496240"
          alt="logo"
          height="100"
        />
      </div>
    </ Col>
    <Col>
      <span><b>{user.firstname}</b> </span>
      <span><b>{user.lastname}</b> </span>
      <br /><br />
      <span>Email : {user.email}</span><br />
      <span>A rejoit le : {user.creationDate}</span>
      
    </ Col>
    </Row>
*/