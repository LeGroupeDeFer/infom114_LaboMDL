import React, { useState } from "react";

import Button from 'react-bootstrap/Button';
import Card from 'react-bootstrap/Card';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import Modal from 'react-bootstrap/Modal';
import InputGroup from 'react-bootstrap/InputGroup';
import FormControl from 'react-bootstrap/FormControl';
import Form from 'react-bootstrap/Form';


const Tag = ({label, deleteTag, updateTag}) => {
  const [modalShow, setModalShow] = useState(false);

  return (
    <>
    <Card style={{ width: '100vw' }}>
      <Card.Body>
        <Container>
          <Row>
            <Col>
              <Card.Title >{label}</Card.Title>
            </Col>

            <Col md="auto">
              <Button variant="secondary" onClick={() => setModalShow(true)} >Update</Button>
              <Button variant="danger" value={label} onClick={deleteTag}>Delete</Button>
            </Col>
          </Row>
        </Container>
      </Card.Body>
    </Card>

    <UpdateTagModal
        show={modalShow}
        onHide={() => setModalShow(false)}
        label={label}
        updateTag={updateTag}
      />
    </>
  )
}

function UpdateTagModal(props) {
  //console.log({...props});
  const [newLabel, setNewLabel] = useState("");

  const handleEdit = (e) => {
    e.preventDefault();

    if (!newLabel)
      return; 
    
    props.updateTag(newLabel, props.label);
    setNewLabel("");
  }
  
  return (
    <Modal
      onHide={props.onHide}
      show={props.show}
      size="lg"
      aria-labelledby="contained-modal-title-vcenter"
      centered
    >
      <Modal.Header closeButton>
        <Modal.Title id="contained-modal-title-vcenter">
          Modifier le tag
        </Modal.Title>
      </Modal.Header>
      <Modal.Body>

      <InputGroup>
        <FormControl
          placeholder="Modifier le nom"
          onChange={e => setNewLabel(e.target.value)}
        />

        <Form onSubmit={handleEdit}>
          <InputGroup.Append>
            <Button variant="outline-secondary" type="submit">Modifier</Button>
          </InputGroup.Append>
        </Form>

      </InputGroup>
      </Modal.Body>
    </Modal>
  );
}

export default Tag;