import React, { useState } from "react";

import Container from 'react-bootstrap/Container';
import Button from 'react-bootstrap/Button';
import Card from 'react-bootstrap/Card';

const Tag = ({label, deleteTag}) => {
  
    return (
      <>
      <Card style={{ width: '100vw' }}>
        <Card.Body>
          <Card.Title>{label}</Card.Title>
          <Button variant="danger" value={label} onClick={deleteTag}>Delete</Button>
        </Card.Body>
      </Card>
      </>
    );
  }

  export default Tag;