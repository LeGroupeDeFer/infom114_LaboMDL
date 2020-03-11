import React from 'react';
import ListGroup from 'react-bootstrap/ListGroup';
import { NavLink } from 'react-router-dom';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { faDoorOpen } from '@fortawesome/free-solid-svg-icons';
import { useLocation } from 'react-router-dom';
import { useAuth } from '../context/authContext';


function Sidebar({ open, links }) {

  const location = useLocation();
  const { user } = useAuth();
  if (['/login', '/register'].includes(location.pathname))
    return <></>;

  return (
    <nav className='sidebar p-2'>

      {/* Logo */}
      <div className="sidebar-logo">
        <img src="https://via.placeholder.com/50" alt="logo" />
      </div>

      {/* Menu */}
      <ListGroup
        variant="flush"
        className="sidebar-nav w-100 text-center"
      >
        {links.map(({ name, path, icon }, i) => (
          <NavLink exact to={path} key={i}>
            <ListGroup.Item>
              <Icon icon={icon} />
            </ListGroup.Item>
          </NavLink>
        ))}
        {user ? (
          <NavLink
            variant="danger"
            exact
            to="/logout"
            className="sidebar-exit"
          >
            <ListGroup.Item>
              <Icon icon={faDoorOpen} />
            </ListGroup.Item>
          </NavLink>
        ) : <></>}
      </ListGroup>

    </nav>
  );
}

Sidebar.defaultProps = {
  links: []
};


export default Sidebar;
