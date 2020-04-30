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

const trace = x => { console.log(x); return x; }

const TagsPage = () => {

  const [tags, setTags] = useState([]);
  const [deletedTag, setDeletedTag] = useState(null);
  const [getPromise, setGetPromise] = useState(null);
  const [delPromise, setDelPromise] = useState(null);
  const [notification, setNotification] = useState("");
  //value of form input
  const [input, setInput] = useState("");
  const Notification = () => notification === "" ? <></> : <TagToast text={notification}/>;
  
  useEffect(() => {
    setGetPromise(api.tags());
  }, []);

  // Get the tags
  useEffect(() => {    
    if (!getPromise)
      return;
    let isRendering = false;
    // On peut faire des changements d'état ici.

    getPromise.then(data => {
      if (!isRendering) 
        if (input.length) {
          setTags([...tags, {label: input}]);
          setInput("");
        }
        else {
          setTags(data.tags);
        }
    }).finally(() => setGetPromise(null));

    // A partir d'ici on ne peut plus.
    return () => isRendering = true;
  }, [getPromise]);

  // Delete a tag
  useEffect(() => {
    if (!delPromise)
      return;
    let isRendering = false;

    delPromise.then(() => {
      if (!isRendering) {
        const remainingTags = tags.filter(t => t.label != deletedTag);
        setTags(remainingTags);
      }
    }).catch(e => {
      if (!isRendering) {
        setNotification("");
        setNotification(e.message);
      }
    }).finally(() => {
      if (!isRendering) {
        setDelPromise(null);
        setDeletedTag(null);
      }
    });

    return () => isRendering = true;
  }, [delPromise]);

  //Handle adding tags to db and hook tags
  const addTag = (label) => {
    const sendTag = async (tag) => {
      await api.tag.add(tag).then((answer) => {
        // FIXME - Need the actual id
        const id = tags.map(t => t.id).reduce((a, i) => a > i ? a : i, 0) + 1;
        const newTags = [...tags, {label, id}];
        setTags(newTags);
      }).catch((error) => {
        setNotification("");
        setNotification(error.message);
      });
    }
    //send data to server
    sendTag(label);
  };

  const onDelete = e => {
    e.preventDefault();
    setDeletedTag(e.target.value);
    setDelPromise(api.tag.remove(e.target.value));
  }

  return (
      <>
        <Notification />
        <AddForm addTag={addTag}/>
        <br />

        {tags.length 
        ? tags.map((tag, i) => {
          return (
            <Row key={tag.id} className="mb-3">
              <Tag name={tag.label} deleteTag={onDelete} setNotification={setNotification}></Tag>
            </Row>
          )
        })
        : <h1>No tags</h1>
        }
      </>
  );
}

export default Admin;