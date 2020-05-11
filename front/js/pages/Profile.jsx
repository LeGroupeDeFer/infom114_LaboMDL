import React, { useEffect, useState } from 'react';

import Container from 'react-bootstrap/Container';
import Button from 'react-bootstrap/Button';
import ButtonGroup from 'react-bootstrap/ButtonGroup';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import OverlayTrigger from 'react-bootstrap/OverlayTrigger';
import Tooltip from 'react-bootstrap/Tooltip';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import 'regenerator-runtime';

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
      res.forEach(post => {
        post.answers = [];
      }); //FIXME - If the post is a poll, answers Array is missing 
      setPosts(res);
    };
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
      <Container fluid>
        <Row>
          <Col xs={12} md={4}>
            <div className="fixed-info">
              <hr />
              <h2 className="mb-3 mt-3">
                <span className=" mr-3"><Icon icon="user" /> <b>Profil utilisateur</b></span>
              </h2>
              {
                user?<User user={user} />:<></>
              }       
              <hr />
            </div>
          </Col>
          <Col xs={12} md={8}>
            <hr />
            {
              posts.map(post => {
                return (<><Post post={post} isPreview /><hr /></>);
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
      <Col xs={10} md={11}></Col>
      <Col md={1} xs={2}>
        <ButtonGroup className="kind-section d-flex justify-content-between">
          <OverlayTrigger
            key={1}
            placement="bottom"
            overlay={<Tooltip>Se déconnecter</Tooltip>}
          >
            <a
              key={1}
              href="/logout"
              className={'kind-choice'}
            >
              <Icon icon="door-open" />
            </a>
          </OverlayTrigger>
        </ButtonGroup>
      </Col>
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
            className="profile-pic"
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