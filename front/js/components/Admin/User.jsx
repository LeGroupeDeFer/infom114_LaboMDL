import React, {useState, useEffect} from "react";
import api from '../../lib/api';

import Button from 'react-bootstrap/Button';
import Card from 'react-bootstrap/Card';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import Modal from 'react-bootstrap/Modal';

const User = ({user, roles}) => {

    const [modalShow, setModalShow] = useState(false);

    return (
        <>
        <Card style={{ width: '100vw' }}>
          <Card.Body>
            <Container>
              <Row>
                <Col>
                  <Card.Title>{user.lastname} {user.firstname}</Card.Title>
                </Col>
                <Button variant="primary" className="mr-3" onClick={() => setModalShow(true)}>Gérer les roles</Button>    
                <Button variant="secondary">Voir profil</Button>
              </Row>
            </Container>
          </Card.Body>
        </Card>

        <EditModal
        user={user}
        roles={roles}
        show={modalShow}
        onHide={() => setModalShow(false)}
        handleClose={() => setModalShow(false)}
        />
        </>
    )
}

const EditModal = ({user, show, onHide, roles, handleClose}) => {
    console.log(roles)

    const [userRoles, setUserRoles] = useState(user.roles);
    
    const handleRemove = (e) => {
      console.log(e.target.value);
    }

    const handleAdd = (e) => {
      console.log(e.target.value);
    }


    return (
        <Modal
        onHide={onHide}
        show={show}
        size="lg"
        aria-labelledby="contained-modal-title-vcenter"
        centered
        >
        <Modal.Body>
        <Container>
            <Row>
                <Col>
                    {userRoles.map( (role, i) => {
                        return (
                          <Row className="mt-3">
                          <h3 key={i} className="mr-3">{role.name}</h3>
                          <Button value={role.name} onClick={handleRemove}>Retirer</Button>
                          </Row>
                        )
                    })
                    }
                </Col>
                <Col>
                    {user.lastname} {user.firstname}
                </Col>
                <Col>
                    {roles.map( (role, i) => {
                        return (
                          <Row className="mt-3">
                          <h3 key={i} className="mr-3">{role.name}</h3>
                          <Button value={role.name} onClick={handleAdd}>Ajouter</Button>
                          </Row>
                        )
                    })
                    }
                </Col>
            </Row>
        </Container>
        </Modal.Body>
        </Modal>
        );
}

export default User;