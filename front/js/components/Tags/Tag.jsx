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


const Tag = ({name, deleteTag, setNotification}) => {
  const [modalShow, setModalShow] = useState(false);
  const [label, setLabel] = useState(name);

  const handleEdit = async (oldLabel, newLabel) => {

    const update = async (oldLabel, newLabel) => {
      let result = await api.tag.edit(oldLabel, newLabel);
      return(result);
    }

    await update(oldLabel, newLabel).then((answer) => {   
      if (Object.keys(answer).length === 0 && answer.constructor === Object) {
        setLabel(newLabel);
      }
    }).catch((error) =>{
      setNotification("");
      setNotification(error.message);
    });
  };


  return (
    <>
    <Card style={{ width: '100vw' }}>
      <Card.Body>
        <Container>
          <Row>
            <Col>
              <Card.Title>{label}</Card.Title>
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
        updateTag={handleEdit}
        handleClose={() => setModalShow(false)}
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
    
    props.updateTag(props.label, newLabel);
  }
  
  return (
    <Modal
      onHide={props.onHide}
      show={props.show}
      size="lg"
      aria-labelledby="contained-modal-title-vcenter"
      centered
    >
      <Modal.Body>
        <InputGroup>
          <InputGroup.Prepend>
            <InputGroup.Text id="basic-addon1">{props.label}</InputGroup.Text>
          </InputGroup.Prepend>
          <FormControl
            placeholder="Modifier le tag"
            onChange={e => setNewLabel(e.target.value)}
          />

          <Form onSubmit={handleEdit}>
            <InputGroup.Append>
              <Button variant="outline-secondary" type="submit" onClick={props.handleClose}>Modifier</Button>
            </InputGroup.Append>
          </Form>

        </InputGroup>
      </Modal.Body>
    </Modal>
  );
}

export default Tag;