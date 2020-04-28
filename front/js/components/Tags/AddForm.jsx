import React, { useState } from 'react';


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
      <form onSubmit={handleSubmit}>
      <label>
        Entrer un tag :
        <input className="input" type="text" value={value} onChange={e => setValue(e.target.value)} />
      </label>
      <input type="submit" value="Envoyer" />
    </form>
    );
  } 

export default AddForm; 