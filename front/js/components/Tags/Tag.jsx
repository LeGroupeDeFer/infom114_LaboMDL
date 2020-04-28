import React, { useState } from "react";

import Container from 'react-bootstrap/Container';
import Button from 'react-bootstrap/Button';

const Tag = ({label, deleteTag}) => {
  
    return (
      <>
      <p>{label}</p>
      <Button className="btn btn-danger ml-3" value={label} onClick={deleteTag}>
        Supprimer
      </Button>
      </>
    );
  }

  export default Tag;