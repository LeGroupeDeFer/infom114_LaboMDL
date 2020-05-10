import React, { useState } from 'react';
import api from '../../lib/api';
import { prevent } from '../../lib';

import Button from 'react-bootstrap/Button';
import Card from 'react-bootstrap/Card';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import Modal from 'react-bootstrap/Modal';
import InputGroup from 'react-bootstrap/InputGroup';
import FormControl from 'react-bootstrap/FormControl';
import Form from 'react-bootstrap/Form';

import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { faPen, faTrashAlt } from '@fortawesome/free-solid-svg-icons';

const Tag = ({ name, deleteTag, setNotification, tags, setTags }) => {
  const [modalShow, setModalShow] = useState(false);
  const [label, setLabel] = useState(name);

  const handleEdit = (oldLabel, newLabel) => {
    const update = async (oldLabel, newLabel) => {
      let result = await api.tag.edit(oldLabel, newLabel);
      return result;
    };

    //Update the name of the tag
    update(oldLabel, newLabel)
      .then((answer) => {
        let tmp = Array.from(tags);
        tmp.map((tag) => {
          if (tag.label === oldLabel) {
            tag.label = newLabel;
          }
        });
        setLabel(newLabel);
        setTags(tmp);
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
  };

  // e => e.preventDefault() || setModalShow(true)

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
                <a
                  className="footer-primary-btn mr-3"
                  href="#"
                  onClick={() => setModalShow(true)}
                >
                  <Icon icon={faPen} className="fa-primary mr-1" />
                  <span className="text-muted">Renommer</span>
                </a>
                <a
                  className="post-footer-btn mr-3"
                  href="#"
                  onClick={() => deleteTag(label)}
                >
                  <Icon icon={faTrashAlt} className="fa-danger mr-1" />
                  <span className="text-muted">Supprimer</span>
                </a>
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
  );
};

function UpdateTagModal(props) {
  const [newLabel, setNewLabel] = useState('');

  const handleEdit = (e) => {
    e.preventDefault();

    if (!newLabel) return;

    props.updateTag(props.label, newLabel);
  };

  return (
    <Modal
      onHide={props.onHide}
      show={props.show}
      size="lg"
      aria-labelledby="contained-modal-title-vcenter"
      centered
    >
      <Modal.Header>Modifier le nom du tag</Modal.Header>
      <Modal.Body>
        <InputGroup>
          <InputGroup.Prepend>
            <InputGroup.Text id="basic-addon1">{props.label}</InputGroup.Text>
          </InputGroup.Prepend>
          <FormControl
            placeholder="Modifier le tag"
            onChange={(e) => setNewLabel(e.target.value)}
          />

          <Form onSubmit={handleEdit}>
            <InputGroup.Append>
              <Button
                variant="outline-secondary"
                type="submit"
                onClick={props.handleClose}
              >
                Modifier
              </Button>
            </InputGroup.Append>
          </Form>
        </InputGroup>
      </Modal.Body>
    </Modal>
  );
}

export default Tag;
