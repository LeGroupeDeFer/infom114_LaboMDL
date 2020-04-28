import React, { useState } from 'react';

import InputGroup from 'react-bootstrap/InputGroup';
import Button from 'react-bootstrap/Button';
import FormControl from 'react-bootstrap/FormControl';
import Form from 'react-bootstrap/Form';
// Add tag Form
const AddForm = ({addTag}) => {

    const [value, setValue] = useState("");
  
    const handleSubmit = (e) => {
      e.preventDefault();
  
      if (!value)
        return; 
      
      addTag(value);
  
      setValue("");
      
    }
  
    return (

      <InputGroup>
      <FormControl
        placeholder="Ajouter un tag"
        value={value} 
        onChange={e => setValue(e.target.value)}
      />

      <Form onSubmit={handleSubmit}>
        <InputGroup.Append>
          <Button variant="outline-secondary" type="submit">Ajouter</Button>
        </InputGroup.Append>
      </Form>

      </ InputGroup>

    );
  } 

export default AddForm; 