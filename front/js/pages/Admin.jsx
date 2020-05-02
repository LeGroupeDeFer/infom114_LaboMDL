import React, { useState, useEffect } from 'react';

import Container from 'react-bootstrap/Container';
import Button from 'react-bootstrap/Button';
import ButtonGroup from 'react-bootstrap/ButtonGroup';
import Row from 'react-bootstrap/Row';

import Tag from '../components/Admin/Tag';
import Role from '../components/Admin/Role';
import Toast from '../components/Admin/Notification';
import AddForm from '../components/Admin/AddForm';
import User from '../components/Admin/User';


import api from '../lib/api';
import 'regenerator-runtime';


function Admin(props) {

  const menuList = ['tags', 'roles', 'users', 'reporting']
  const [currentMenu, setCurrentMenu] = useState('tags');
  const Page = () => {
    if (currentMenu == 'tags') {
      return <TagsPage />;
    }
    else if (currentMenu == 'users') {
      return <UsersPage />;
    }
    else if (currentMenu == 'reporting') {
      return <p>reporting page</p>;
    }
    else {
      return <RolesPage />;
    }
  } 

  return (
    <Container
      style={{
        position: 'relative',
      }}>
      <br />
      <div style={{ position: 'absolute', top: 0, right: 0, 'zIndex': 1 }}></div>

      <Row className='justify-content-md-center'>
        <MenuBar onClick={setCurrentMenu} currentMenu={currentMenu} menuList={menuList}/>
      </Row>
      <br />
      <div>
        <Page />
      </div>
    </Container>
  );
}

const MenuBar = ({ currentMenu, onClick, menuList }) => {
  
  return (
    <ButtonGroup id='menu-bar'>
      { menuList.map((menu, i) => {
        return <Button key={i} variant="secondary" className={currentMenu == menu ? 'active' : ''} onClick={() => onClick(menu)}>{menu}</Button>
      })
      }
    </ButtonGroup>
  );

};

const UsersPage = () => {

  const [users, setUsers] = useState([]);
  
  useEffect(() => {
    const fetchUsers = async () => {
      let users = await api.users();
      setUsers(users);
    }

    fetchUsers();
  }, [])

  return (
  <>
      {users.length
        ? users.map((user) => {
          return (
            <Row key={user.id} className="mb-3">
              <User name={user.firstname}/>
            </Row>
          )
        })
        : <h1>No users</h1>
      }
  </>
  )
}

const RolesPage = () => {

  const [roles, setRoles] = useState([]);
  const [users, setUsers] = useState([]);
  const [notification, setNotification] = useState("");
  const [capabilities, setCapabilities] = useState([]);

  const Notification = () => notification === "" ? <></> : <Toast text={notification} />;

  useEffect(() => {

    const fetchRoles = async () => {
      let roles = await api.roles()
      setRoles(roles);
      console.log(roles);
    }

    const fetchCapabilities = async () => {
      let capabilities = await api.capabilities();
      setCapabilities(capabilities);
      console.log(capabilities);
      
    }
    
    fetchRoles();
    fetchCapabilities();
  }, [])

  //Gets all information about a role
  //Useful to get missing information when creating a new role 
  const fetchRoleInformation = async (roleToModify) => {

    const roleInformation = async () => {
      let result = api.roles();
      return result;
    }
    let roles = await roleInformation();
    return roles.filter(role => role.name === roleToModify.name )[0];
  }


  //Add a new role 
  const addRole = (roleName) => {

    const sendRole = async (roleName) => {
      await api.roles.add(roleName, "#8fd5a6", "").then(async answer => {
        let role = await fetchRoleInformation({ name: roleName });
        const newRoles = [...roles, { color: role.color, id: role.id, name: role.name, capabilities: role.capabilities }];
        setRoles(newRoles); c

      }).catch((error) => {
        let reason = error.reason == null ? "La demande n'a pu être traitée" : error.reason;
        setNotification('');
        setNotification(reason);
        console.log(error);
        
      });
    }
    sendRole(roleName);
  }

  const handleDelete = (roleId) => {

    const deleteRole = async (id) => {

      await api.roles.delete(id).then((answer) => {
        let remainingRoles = roles.filter(remainingRole => remainingRole.id !== id);
        setRoles(remainingRoles);  //remainingRoles is correct but it does not rerender well
      }).catch((error) => {
        let reason = error.reason == null ? "La demande n'a pu être traitée" : error.reason
        setNotification("");
        setNotification(reason);
        console.log(error);
      });
    }
    deleteRole(roleId);
  }

  return (
    <>
      <Notification />
      <br />

      <AddForm add={addRole} />
      <br />

      {roles.length
        ? roles.map((role, i) => {
          return (
            <Row key={role.id} className="mb-3">
              <Role roleId={role.id} roleName={role.name} roleColor={role.color} roleCapabilities={role.capabilities}
                deleteRole={handleDelete} setNotification={setNotification} />
            </Row>
          )
        })
        : <h1>No roles</h1>
      }
    </>
  );
};

const TagsPage = () => {
  const [tags, setTags] = useState([]);
  const [deletedTag, setDeletedTag] = useState(null);
  const [getPromise, setGetPromise] = useState(null);
  const [delPromise, setDelPromise] = useState(null);
  const [notification, setNotification] = useState("");
  //value of form input
  const [input, setInput] = useState("");

  const Notification = () => notification === "" ? <></> : <Toast text={notification} />;


  useEffect(() => {
    setGetPromise(api.tags());
    console.log('getting tags for first time')
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
          setTags([...tags, { label: input }]);
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
        let reason = e.reason == null ? "La demande n'a pu être traitée" : error.reason
        setNotification("");
        setNotification(reason);
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
        const newTags = [...tags, { label, id }];
        setTags(newTags);
      }).catch((error) => {
        let reason = error.reason == null ? "La demande n'a pu être traitée" : error.reason; 
        setNotification("");
        setNotification(reason);
        console.log(error);
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
      <br />

      <AddForm add={addTag} />
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