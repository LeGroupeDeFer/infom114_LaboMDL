import React, {useState} from "react";
import api from '../../lib/api';

import Button from 'react-bootstrap/Button';
import Card from 'react-bootstrap/Card';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import Modal from 'react-bootstrap/Modal';
import InputGroup from 'react-bootstrap/InputGroup';
import FormControl from 'react-bootstrap/FormControl';
import Form from 'react-bootstrap/Form';


const User = ({user}) => {

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
                <Col md="auto">
                  <Button variant="secondary" className="mr-3" onClick={() => setModalShow(true)} >Modifier</Button> 
                  <Button>Voir profil</Button>
                </Col>
              </Row>
            </Container>
          </Card.Body>
        </Card>
    
        <UpdateUserModal
            show={modalShow}
            onHide={() => setModalShow(false)}
            handleClose={() => setModalShow(false)}
          />
        </>
    )
}

const UpdateUserModal = (props) => {
    return (
        <Modal
          onHide={props.onHide}
          show={props.show}
          size="lg"
          aria-labelledby="contained-modal-title-vcenter"
          centered
        >
          <Modal.Body>
            <p>Roles</p>
            <Form>
                <Form.Check 
                    type="checkbox"
                    label="TATATATAAAA"
                />  
                </Form>
          </Modal.Body>
        </Modal>
      );
}

export default User;