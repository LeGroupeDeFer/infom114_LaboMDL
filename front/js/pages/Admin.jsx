import React, { useState, useEffect } from 'react';

import Container from 'react-bootstrap/Container';
import Button from 'react-bootstrap/Button';
import ButtonGroup from 'react-bootstrap/ButtonGroup';
import Row from 'react-bootstrap/Row';
import Col from 'react-bootstrap/Col';
import Card from 'react-bootstrap/Card';

import Tag from '../components/Admin/Tag';
import Role from '../components/Admin/Role';
import Toast from '../components/Admin/Notification';
import AddForm from '../components/Admin/AddForm';
import User from '../components/Admin/User';

import { FaTags, FaUsers, FaChartLine, FaClipboardCheck, FaTag } from 'react-icons/fa';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';

import OverlayTrigger from 'react-bootstrap/OverlayTrigger';
import Tooltip from 'react-bootstrap/Tooltip';

import { ResponsiveContainer, ComposedChart, RadarChart, PieChart, Pie, Line, PolarGrid, PolarAngleAxis, PolarRadiusAxis, Radar, CartesianGrid, XAxis, YAxis, Cell, Tooltip as RechartsTooltip, Legend, Bar } from 'recharts';

import api from '../lib/api';
import 'regenerator-runtime';
import clsx from 'clsx';


function Admin(props) {

  const menuList = ['tags', 'roles', 'users', 'reporting'];
  const [currentMenu, setCurrentMenu] = useState('reporting');

  const Page = () => {
    if (currentMenu === 'tags') {
      return <TagsPage />;
    }
    else if (currentMenu === 'users') {
      return <UsersPage />;
    }
    else if (currentMenu === 'reporting') {
      return <ReportingPage />;
    }
    else {
      return <RolesPage />;
    }
  }

  return (
    <>
      <Container fluid className="menu-bar-container py-2">
        <MenuBar onClick={setCurrentMenu} currentMenu={currentMenu} menuList={menuList} className="menu-bar" />
      </Container>
      <br />
      <br />
      <br />
      <Container>
        <Page />
      </Container>
    </>
  );
};

const MenuBar = ({ currentMenu, onClick, menuList }) => {

  const icons = [<FaTags />, <FaClipboardCheck />, <FaUsers />, <FaChartLine />];
//<a key={i} className={currentMenu == menu ? 'active mr-5' : 'mr-5'} onClick={() => onClick(menu)}>{icons[i]}</a>

  return (
    
    <Row>
      <Col xs={2}></Col>
      <Col xs={8}>
        <ButtonGroup className="kind-section d-flex justify-content-between" >
          {menuList.map((menu, i) => {
            return (
              <OverlayTrigger
                placement="bottom"
                overlay={<Tooltip>{menu}</Tooltip>}
              >
                <Button
                  key={i}
                  className={clsx('kind-choice', menu === currentMenu && 'active')}
                  onClick={() => onClick(menu)}
                >
                  {icons[i]}
                </Button>

              </OverlayTrigger>
            );
          })
          }
        </ButtonGroup>
      </Col>
      <Col xs={2}></Col>
    </Row>
  );
};

const UsersPage = () => {

  const [users, setUsers] = useState([]);
  const [roles, setRoles] = useState([]);

  const [notification, setNotification] = useState("");

  const Notification = () => notification === "" ? <></> : <Toast text={notification} />;

  useEffect(() => {
    const fetchUsers = async () => {
      let users = await api.users();
      setUsers(users);
    }

    const fetchRoles = async () => {
      let roles = await api.roles()
      setRoles(roles);
    }

    fetchUsers();
    fetchRoles();

  }, [])

  return (
    <>
      <Notification />
      <br />
      {users.length
        ? users.map((user) => {
          return (
            <Row key={user.id} className="mb-3">
              <User user={user} roles={roles} setNotification={setNotification} />
            </Row>
          )
        })
        : <h1>No users</h1>
      }
    </>
  )
};

const ReportingPage = () => {

  const colors = ["#A0C55F", "#0D6759", "#1B4079", "#FC440F"];
  const [graphData, setGraphData] = useState({connect:[], active:[], tag:[], post:[]});

  //API call here
  useEffect(() => {
    fetchData().then( answer => {
      console.log(answer);
      setGraphData(answer);
    });
  }, []);

  const fetchData = async () => {
    let usersData = await api.users.report();
    let connect = [
      {
        name: "Connecté",
        value: 43
      },
      {
        name: "Déconnecté",
        value: 236
      }
    ];
    let active = [
      {
        name: "Compte activé",
        value: usersData.active
      },
      {
        name: "Compte désactivé",
        value: usersData.total - usersData.active
      }
    ];
    let tag = [
      {
        "tag": "Info",
        "Info": 120,
        "Idée": 110,
        "Sondage": 85,
        "fullMark": 150
      },
      {
        "tag": "Pharma",
        "Info": 98,
        "Idée": 130,
        "Sondage": 56,
        "fullMark": 150
      },
      {
        "tag": "Droit",
        "Info": 86,
        "Idée": 130,
        "Sondage": 120,
        "fullMark": 150
      },
      {
        "tag": "Eco",
        "Info": 99,
        "Idée": 100,
        "Sondage": 54,
        "fullMark": 150
      },
      {
        "tag": "Physics",
        "Info": 85,
        "Idée": 90,
        "Sondage": 67,
        "fullMark": 150
      },
      {
        "tag": "History",
        "Info": 65,
        "Idée": 85,
        "Sondage": 98,
        "fullMark": 150
      }
    ];
    let post = [
      { name: 'Janvier', nouveau: 20, interaction: 124 },
      { name: 'Février', nouveau: 13, interaction: 40 },
      { name: 'Mars', nouveau: 24, interaction: 75 },
      { name: 'Avril', nouveau: 40, interaction: 150 },
      { name: 'Mai', nouveau: 5, interaction: 47 },
      { name: 'Juin', nouveau: 0, interaction: 0 },
      { name: 'Juillet', nouveau: 0, interaction: 0 },
      { name: 'Aout', nouveau: 0, interaction: 0 },
      { name: 'Septembre', nouveau: 0, interaction: 0 },
      { name: 'Octobre', nouveau: 0, interaction: 0 },
      { name: 'Novembre', nouveau: 0, interaction: 0 },
      { name: 'Décembre', nouveau: 0, interaction: 0 }
    ];

    return {connect, active, tag, post};
  };

  return (
    <Container>
      <Row>
        <Col md={4}>
          <Card style={{ padding: '1rem' }}>
            <Card.Title>Utilisateurs</Card.Title>
            <Card.Subtitle className="mb-2 text-muted">Nombre d'utilsateurs actif et non actif</Card.Subtitle>
            <ResponsiveContainer height={300}>
              <PieChart margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
                <Pie data={graphData.active} dataKey="value" nameKey="name" cx="50%" cy="50%" outerRadius={55}>
                  {
                    graphData.active.map((entry, index) => (
                      <Cell key={`cell-${index}`} fill={colors[index]} />
                    ))
                  }
                </Pie>
                <Pie data={graphData.connect} dataKey="value" nameKey="name" cx="50%" cy="50%" innerRadius={65} outerRadius={85} label>
                  {
                    graphData.connect.map((entry, index) => (
                      <Cell key={`cell-${index}`} fill={colors[index+2]} />
                    ))
                  }
                </Pie>
                <Legend />
              </PieChart>
            </ResponsiveContainer>
          </Card>
          <hr />
        </Col>


        <Col md={8}>
          <Card style={{ padding: '1rem' }}>
            <Card.Title>Tags et leur utilisation</Card.Title>
            <Card.Subtitle className="mb-2 text-muted">Montre le nombre de citation par tag ainsi que le type de post associé</Card.Subtitle>
            <ResponsiveContainer height={300}>
              <RadarChart outerRadius={90} margin={{ top: 5, right: 30, left: 20, bottom: 5 }} data={graphData.tag}>
                <PolarGrid />
                <PolarAngleAxis dataKey="tag" />
                <PolarRadiusAxis angle={30} domain={[0, 150]} />
                <Radar name="Informationnels" dataKey="Info" stroke={colors[2]} fill={colors[2]} fillOpacity={0.2} />
                <Radar name="Proposition d'idée" dataKey="Idée" stroke={colors[1]} fill={colors[1]} fillOpacity={0.4} />
                <Radar name="Sondages" dataKey="Sondage" stroke={colors[0]} fill={colors[0]} fillOpacity={0.6} />
                <Legend />
              </RadarChart>
            </ResponsiveContainer>
          </Card>
          <hr />
        </Col>


        <Col md={12}>
          <Card style={{ padding: '1rem' }}>
            <Card.Title>Postes créés sur l'année</Card.Title>
            <Card.Subtitle className="mb-2 text-muted">Nombre de nouveaux postes depuis le début de l'année, ainsi que l'interaction liée</Card.Subtitle>
            <ResponsiveContainer height={250}>
              <ComposedChart data={graphData.post} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
                <XAxis dataKey="name" />
                <YAxis />
                <RechartsTooltip />
                <Legend />
                <CartesianGrid stroke="#f5f5f5" />
                <Bar dataKey="nouveau" barSize={20} fill={colors[0]} />
                <Line type="monotone" dataKey="interaction" stroke={colors[1]} />
              </ComposedChart>
            </ResponsiveContainer>
          </Card>
        </Col>
      </Row>
    </Container>
  );
}

const RolesPage = () => {

  const [roles, setRoles] = useState([]);
  const [notification, setNotification] = useState("");
  const [capabilities, setCapabilities] = useState([]);

  const Notification = () => notification === "" ? <></> : <Toast text={notification} />;

  useEffect(() => {

    const fetchRoles = async () => {
      let roles = await api.roles()
      setRoles(roles);
    }

    const fetchCapabilities = async () => {
      let capabilities = await api.capabilities();
      setCapabilities(capabilities);
    }

    fetchRoles();
    fetchCapabilities();
  }, []);

  //Gets all information about a role
  //Useful to get missing information when creating a new role 
  const fetchRoleInformation = async (roleToModify) => {

    const roleInformation = async () => {
      let result = api.roles();
      return result;
    }
    let roles = await roleInformation();
    return roles.filter(role => role.name === roleToModify.name)[0];
  }

  //Add a new role 
  const addRole = (roleName) => {

    const sendRole = async (roleName) => {
      await api.roles.add(roleName, "#8fd5a6", "").then(async answer => {
        let role = await fetchRoleInformation({ name: roleName });
        const newRoles = [...roles, { color: role.color, id: role.id, name: role.name, capabilities: role.capabilities }];
        setRoles(newRoles);

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
                deleteRole={handleDelete} setNotification={setNotification} allCapabilities={capabilities} />
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
        let reason = e.reason == null ? "La demande n'a pu être traitée" : e.reason
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

  const onDelete = (label) => {
    setDeletedTag(label);
    setDelPromise(api.tag.remove(label));
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
              <Tag name={tag.label} deleteTag={onDelete} setNotification={setNotification} tags={tags} setTags={setTags} ></Tag>
            </Row>
          )
        })
        : <h1>No tags</h1>
      }
    </>
  );
};

export default Admin;