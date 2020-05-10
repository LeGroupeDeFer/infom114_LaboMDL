import React, { useState, useEffect } from 'react';
import api from '../../lib/api';
import { prevent } from '../../lib';

import Card from 'react-bootstrap/Card';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import Modal from 'react-bootstrap/Modal';
import Form from 'react-bootstrap/Form';

import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { faEdit, faUser } from '@fortawesome/free-solid-svg-icons';

const User = ({ user, roles, setNotification }) => {
  const [modalShow, setModalShow] = useState(false);

  return (
    <>
      <Card style={{ width: '100vw' }}>
        <Card.Body>
          <Container>
            <Row>
              <Col>
                <Card.Title>
                  {user.lastname} {user.firstname}
                </Card.Title>
              </Col>
              <Col md="auto">
                <a
                  className="footer-primary-btn mr-3"
                  href="#"
                  onClick={() => setModalShow(true)}
                >
                  <Icon icon={faEdit} className="fa-primary mr-1" />
                  <span className="text-muted">Gérer les roles</span>
                </a>
                <a className="post-footer-btn mr-3" href="#">
                  <Icon icon={faUser} className="fa-primary mr-1">
                    Voir profil
                  </Icon>
                  <span className="text-muted">Voir profil</span>
                </a>
              </Col>
            </Row>
          </Container>
        </Card.Body>
      </Card>

      <EditModal
        user={user}
        roles={roles}
        show={modalShow}
        onHide={() => setModalShow(false)}
        setNotification={setNotification}
      />
    </>
  );
};

const containsID = (list, id) => {
  for (var i = 0; i < list.length; i++) {
    if (list[i].id == id) {
      return true;
    }
  }
  return false;
};

const EditModal = ({ user, show, onHide, setNotification }) => {
  const userRoles = user.roles;
  const [rolesList, setRolesList] = useState([]);

  useEffect(() => {
    const fetchRoles = async () => {
      let roles = await api.roles();
      roles.forEach((role) => {
        if (containsID(userRoles, role.id)) {
          role.assigned = true;
        } else {
          role.assigned = false;
        }
      });
      setRolesList(roles);
    };

    fetchRoles();
  }, []);

  const handleEdit = (e) => {
    const removeRoleFromUser = async (userID, roleID) => {
      let res = await api.users.removeRole(userID, roleID);
      return res;
    };

    const addRoleToUser = async (userID, roleID) => {
      let res = await api.users.addRole(userID, roleID);
      return res;
    };

    const checked = e.target.checked;
    const id = parseInt(e.target.id);

    if (checked) {
      addRoleToUser(user.id, e.target.id)
        .then((answer) => {
          let tmp = Array.from(rolesList);
          tmp.map((role) => {
            if (role.id == id) {
              role.assigned = checked;
            }
          });

          setRolesList(tmp);
        })
        .catch((error) => {
          let reason =
            error.reason == null
              ? "La demande n'a pu être traitée"
              : error.reason;
          setNotification('');
          setNotification(reason);
          console.log(error);
        });
    } else {
      removeRoleFromUser(user.id, e.target.id)
        .then((answer) => {
          let tmp = Array.from(rolesList);
          tmp.map((role) => {
            if (role.id == id) {
              role.assigned = checked;
            }
          });
          setRolesList(tmp);
        })
        .catch((error) => {
          let reason =
            error.reason == null
              ? "La demande n'a pu être traitée"
              : error.reason;
          setNotification('');
          setNotification(reason);
          console.log(error);
        });
    }
  };

  return (
    <Modal
      onHide={onHide}
      show={show}
      size="lg"
      aria-labelledby="contained-modal-title-vcenter"
      centered
    >
      <Modal.Header closeButton>
        <Modal.Title>
          Utilisateur: {user.lastname} {user.firstname}
        </Modal.Title>
      </Modal.Header>
      <Modal.Body>
        <hr />
        {rolesList.map((role) => {
          return (
            <>
              <Row key={role.id}>
                <Col>{role.name}</Col>
                <Col md="auto">
                  <Form.Check
                    type="switch"
                    id={role.id}
                    label={' '}
                    checked={role.assigned}
                    onClick={handleEdit}
                  />
                </Col>
              </Row>
              <hr />
            </>
          );
        })}
      </Modal.Body>
    </Modal>
  );
};

export default User;
