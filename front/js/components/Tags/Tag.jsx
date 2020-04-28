import React from "react";

import Button from 'react-bootstrap/Button';
import Card from 'react-bootstrap/Card';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';


const Tag = ({label, deleteTag}) => {
  
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

              <div id="coucou">
              </div>

            </Col>
          </Row>
        </Container>
      </Card.Body>
    </Card>
    </>
  )

  }

  export default Tag;