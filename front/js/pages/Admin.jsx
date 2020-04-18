import React, { useState, useEffect } from 'react';

import Container from 'react-bootstrap/Container';
import Button from 'react-bootstrap/Button';
import ButtonGroup from 'react-bootstrap/ButtonGroup';
import Row from 'react-bootstrap/Row';
import Modal from 'react-bootstrap/Modal';

import api from '../lib/api';
import 'regenerator-runtime';


const MenuBar = ({ currentMenu, onClick }) => {

  return (
    <ButtonGroup id='menu-bar'>
      <Button
        variant='secondary'
        className={currentMenu == 'tag' ? 'active' : ''}
        onClick={() => onClick('tag')}
      >
        Tags
      </Button>
      <Button
        variant='secondary'
        className={currentMenu == 'roles' ? 'active' : ''}
        onClick={() => onClick('roles')}
      >
        Roles
      </Button>
    </ButtonGroup>
  );

}


const RolesPage = () => <h3>roles</h3>;



const TagsPage = () => {

  const [tags, setTags] = useState([]);
  const [promise, setPromise] = useState(null);
  
  //value of form input
  const [input, setInput] = useState("");

  useEffect(() => {
    setPromise(api.tags());
  }, []);

  useEffect(() => {
    if (!promise)
      return;
    let isRendering = false;
    // On peut faire des changements d'état ici.

    promise.then(data => {
      if (!isRendering) 
        if (input.length) {
          setTags([...tags, {label: input}]);
          setInput("");
        }
        else {
          setTags(data.tags);
        }
    });

    // A partir d'ici on ne peut plus.
    return () => isRendering = true;
  }, [promise]);

  //Handle adding tags to db and hook tags
  const addTag = (label) => {
    const sendTag = async (tag) => {
      let result = await api.tag.add(tag);
      return(result);
    }
    
    //send data to server
    let response = sendTag(label);
    
    //handle tag already exists
    if (tags.some(tag => tag.label === label)) {
      alert("Tag already exists")
      return;
    }
    //adding new tag to hook tags
    const newTags = [...tags, {label}];
    setTags(newTags);
  };

  //Handle delete tag button
  const handleDelete = (e) => {
    e.preventDefault()

    const removeTag = async (tag) => {
      let result = await api.tag.remove(tag)
      return(result);
    }

    let response = removeTag(e.target.value);

    //remove tag in hook tag
    let newTags = tags.filter( tag => tag.label !== e.target.value);
    
    setTags(newTags);

  }

  const handleEdit = (e) => {
    console.log(e.target.value);
  }

  return (
      <div>
      {tags.length 
      ? tags.map((tag, i) => {
        return (
          <Row key={i} className="mb-3">
            <Tag label={tag.label} deleteTag={handleDelete} editTag={handleEdit}></Tag>
          </Row>
        )
      })
      : <h1>No tags</h1>
      }
      <AddForm addTag={addTag}/>
      </div>
  );
}

const Tag = ({label, deleteTag, editTag}) => {
  
  return (
    <>
    <p><span id={label}>{label}</span></p>
    <Button className="btn btn-danger ml-3" value={label} onClick={deleteTag}>
      Supprimer
    </Button>
    <Button className="btn btn-primary ml-3" value={label} onClick={editTag}>
      Editer
    </Button>
    </>
  );
}

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


function Admin(props) {

  const [menu, setMenu] = useState('tag');
  const Page = () =>  menu == 'tag' ? <TagsPage /> : <RolesPage />;

  return (
    <Container>
      <br />
      <Row className='justify-content-md-center'>
        <MenuBar onClick={setMenu} currentMenu={menu} />
      </Row>

      <br />
      <Page />

    </Container>
  );
}

export default Admin;