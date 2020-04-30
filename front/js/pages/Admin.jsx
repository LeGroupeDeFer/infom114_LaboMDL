import React, { useState, useEffect } from 'react';

import Container from 'react-bootstrap/Container';
import Button from 'react-bootstrap/Button';
import ButtonGroup from 'react-bootstrap/ButtonGroup';
import Row from 'react-bootstrap/Row';

import Tag from '../components/Tags/Tag';
import TagToast from '../components/Tags/Toast';
import AddForm from '../components/Tags/AddForm';


import api from '../lib/api';
import 'regenerator-runtime';

function Admin(props) {

  const [menu, setMenu] = useState('tag');
  const Page = () =>  menu == 'tag' ? <TagsPage/> : <RolesPage />;

  return (
    <Container
        style={{
          position: 'relative',
        }}>
      <br />
      <div style={{position: 'absolute', top: 0,right: 0, 'z-index':1}}></div>

      <Row className='justify-content-md-center'>
        <MenuBar onClick={setMenu} currentMenu={menu} />
      </Row>
      <br />
      <div> 
        <Page />
      </div>
    </Container>
  );
}

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

};

const RolesPage = () => {

  const [roles, setRoles] = useState([]);
  const [users, setUsers] = useState([]);
  const [capabilities, setCapabilities] = useState([]);

  useEffect(() => {

    const fetchRoles = async () => {
      let roles = await api.roles();
      setRoles(roles);
    }

    const fetchCapabilities = async () => {
      let capabilities = await api.capabilities();
      setCapabilities(capabilities);
    }

    const fetchUsers = async () => {
      let users = await api.users();
      setUsers(users);
    }

    fetchUsers();
    fetchRoles();
    fetchCapabilities();
  }, [])

  return (
    <>
    {roles.map((role, i) => {
      return <Row key={i}>{role.name}</Row>;
    })
    }
    </>
  );
};

const TagsPage = () => {

  const [tags, setTags] = useState([]);
  const [promise, setPromise] = useState(null);
  const [notification, setNotification] = useState("");
  
  //value of form input
  const [input, setInput] = useState("");

  const Notification = () => notification === "" ? <TagToast text={""}/> : <TagToast text={notification}/>;

  
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
      await api.tag.add(tag).then((answer) => {
        const newTags = [...tags, {label}];
        setTags(newTags);
      }).catch((error) => {
        setNotification("");
        setNotification(error.message);
      });
    }
    //send data to server
    sendTag(label);
  };

  //Handle delete tag button
  const handleDelete = (e) => {
    e.preventDefault()

    const removeTag = async (tag) => {
      await api.tag.remove(tag).then((answer) => {
        let newTags = tags.filter( tag => tag.label !== e.target.value);
        setTags(newTags); 
      }).catch((error) => {
        setNotification("");
        setNotification(error.message);
      });
    }
    removeTag(e.target.value);
  }

  return (
      <>
        <Notification />
        <AddForm addTag={addTag}/>
        <br />

        {tags.length 
        ? tags.map((tag, i) => {
          return (
            <Row key={i} className="mb-3">
              <Tag name={tag.label} deleteTag={handleDelete} setNotification={setNotification}></Tag>
            </Row>
          )
        })
        : <h1>No tags</h1>
        }
      </>
  );
}

export default Admin;