import React, { useState, useEffect } from "react";
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

const Role = ({ roleId, roleName, roleColor, roleCapabilities, deleteRole, setNotification, allCapabilities }) => {

  const [renameModalShow, setRenameModalShow] = useState(false);
  const [editModalShow, setEditModalShow] = useState(false);

  const [role, setRole] = useState({ id: roleId, name: roleName, color: roleColor, capabilities: roleCapabilities })

  const handleRename = async (newName) => {

    const update = async (newName) => {
      let result = await api.roles.edit(role.id, newName, role.color, role.capabilities);
      return (result);
    }
    await update(newName).then((answer) => {
      setRole({ id: role.id, name: newName, color: role.color, capabilities: role.capabilities });

    }).catch((error) => {
      let reason = error.reason == null ? "La demande n'a pu être traitée" : error.reason
      setNotification("");
      setNotification(reason);
      console.log(reason);

    });
  };

  return (
    <>
      <Card style={{ width: '100vw' }}>
        <Card.Body>
          <Container>
            <Row>
              <Col>
                <Card.Title>{role.name}</Card.Title>
              </Col>

              <Col md="auto">
                <Button variant="secondary" className="mr-3" onClick={() => setEditModalShow(true)} >Modifier</Button>
                <Button variant="primary" className="mr-3" onClick={() => setRenameModalShow(true)} >Renommer</Button>
                <Button variant="danger" value={role.name} onClick={() => deleteRole(role.id)} >Supprimer</Button>
              </Col>
            </Row>
          </Container>
        </Card.Body>
      </Card>

      <RenameModal
        show={renameModalShow}
        onHide={() => setRenameModalShow(false)}
        name={role.name}
        renameRole={handleRename}
        handleClose={() => setRenameModalShow(false)}
      />
      <EditModal
        show={editModalShow}
        onHide={() => setEditModalShow(false)}
        setNotification={setNotification}
        setRole={setRole}
        handleClose={() => setEditModalShow(false)}
        roleToModify={ role }
        allCapabilities={ allCapabilities }
      />
    </>
  )
}

function RenameModal(props) {
  //console.log({...props});
  const [newName, setNewName] = useState("");

  const handleRename = (e) => {
    e.preventDefault();
    if (!newName)
      return;

    props.renameRole(newName);
  }

  return (
    <Modal
      onHide={props.onHide}
      show={props.show}
      size="lg"
      aria-labelledby="contained-modal-title-vcenter"
      centered
    >
      <Modal.Header>
        Modifier le nom du role
      </Modal.Header>
      <Modal.Body>
        <InputGroup>
          <InputGroup.Prepend>
            <InputGroup.Text id="basic-addon1">{props.name}</InputGroup.Text>
          </InputGroup.Prepend>
          <FormControl
            placeholder="Modifier le nom"
            onChange={e => setNewName(e.target.value)}
          />

          <Form onSubmit={handleRename}>
            <InputGroup.Append>
              <Button variant="outline-secondary" type="submit" onClick={props.handleClose}>Modifier</Button>
            </InputGroup.Append>
          </Form>

        </InputGroup>
      </Modal.Body>
    </Modal>
  );
}

function EditModal({ onHide, show, roleToModify, allCapabilities, setRole, setNotification }) {

  const [capabilities, setCapabilities] = useState([]); //1 checked, 0 unchecked 

  useEffect(() => {
    const getCapabilities = async () => {

      //Add every capacity to the modal
      let tmp = Array.from(capabilities); //in case there is no capability to show
      allCapabilities.forEach(capability => {
        tmp = [...tmp, { id: capability.id, name: capability.name, assigned: false }]
      });

      //Indicate wether or not the capability is assigned to the role
      roleToModify.capabilities.forEach(e => {
        tmp.forEach(capability => {
          if (capability.id === e.id) {
            capability.assigned = true;
          }
        });
      });

      setCapabilities(tmp);
    };
    getCapabilities();


  }, []);

  function handleEdit(button, capability) {
    let tmp = capabilities.map(e => {
      if (e.id === capability.id) {
        let newCapabilities;
        if (e.assigned){
          newCapabilities = roleToModify.capabilities.filter(f => f.id !== e.id);

        }else {
          newCapabilities = [...roleToModify.capabilities, capability]
        }
        //telling the server
        api.roles.edit(roleToModify.id, roleToModify.name, roleToModify.color, newCapabilities).then(answer => {
          e.assigned = !e.assigned;
          setRole({ id: roleToModify.id, name: roleToModify.name, color: roleToModify.color, capabilities: newCapabilities });
        }).catch(error => {
          let reason = error.reason == null ? "La demande n'a pu être traitée" : error.reason;
          setNotification('');
          setNotification(reason);
          console.log(error);
        });
      }
      return e;
    });
    setCapabilities(tmp)
    //capability.assigned = !capability.assigned; //TODO PAS BON, pas modifier la dom
    //api.roles.edit(roleToModify.id, roleToModify.name, roleToModify.color, [...roleToModify.capabilities, capability])
  }

  return (
    <Modal
      onHide={onHide}
      show={show}
      size="lg"
      aria-labelledby="contained-modal-title-vcenter"
      centered
    >
      <Modal.Header>
        Modifier les capabilities du role {roleToModify.name}
      </Modal.Header>
      <Modal.Body>
        {capabilities.map(capability => {
            return (
              <Card>
                <Card.Body>
                  <Row>
                    <Col>
                      {capability.name}
                    </Col>
                    <Col md="auto">

                      <Form.Check 
                        type="switch"
                        id={ capability.id }
                        label={ " " }
                        checked={ capability.assigned }
                        onClick={ e => handleEdit(e, capability) }
                      />
                    </Col>
                  </Row>
                </Card.Body>
              </Card>
            );
          })}
      </Modal.Body>
    </Modal>
  );
}

export default Role;