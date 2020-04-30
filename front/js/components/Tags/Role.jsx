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


const Role = ({roleId, roleName, roleColor, roleCapabilities, deleteRole, setNotification}) => {
    const [renameModalShow, setRenameModalShow] = useState(false);
    const [editModalShow, setEditModalShow] = useState(false);
    const [name, setName] = useState(roleName);
    const [color, setColor] = useState(roleColor);
    const [capability, setCapability] = useState(roleCapabilities);

    const handleRename = async (newName) => {
       
        const update = async (newName) => {
            let result = await api.roles.edit(roleId, newName, color, capability);
            return(result);
        }
        await update(newName).then((answer) => {   
            console.log(answer);
            if (Object.keys(answer).length === 0 && answer.constructor === Object) {
            setName(newName);
            }
        }).catch((error) =>{
            console.log(error);
            setNotification("");
            setNotification(error.message);
        });
    };  
    
    const handleEdit = async () => {
        console.log("TODO");
        
    }

    return (
        <>
            <Card style={{ width: '100vw' }}>
                <Card.Body>
                <Container>
                    <Row>
                    <Col>
                        <Card.Title>{name}</Card.Title>
                    </Col>

                    <Col md="auto">
                        <Button variant="secondary" onClick={() => setEditModalShow(true)} >Modifier</Button> 
                        <Button variant="secondary" onClick={() => setRenameModalShow(true)} >Renommer</Button> 
                        <Button variant="danger" value={name} onClick={() => deleteRole(roleId)} >Supprimer</Button>
                    </Col>
                    </Row>
                </Container>
                </Card.Body>
            </Card>

            <RenameModal
                show={renameModalShow}
                onHide={() => setRenameModalShow(false)}
                name={name}
                renameRole={handleRename}
                handleClose={() => setRenameModalShow(false)}
            />
            <EditModal 
                show={editModalShow}
                onHide={() => setEditModalShow(false)}
                updateRole={handleEdit}
                handleClose={() => setEditModalShow(false)}   
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

function EditModal(props) {
    //console.log({...props});

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
              <InputGroup.Text id="basic-addon2">TODO</InputGroup.Text>
            </InputGroup.Prepend>
            <FormControl
              placeholder="TODO"
            />
  
          </InputGroup>
        </Modal.Body>
      </Modal>
    );
}

export default Role;