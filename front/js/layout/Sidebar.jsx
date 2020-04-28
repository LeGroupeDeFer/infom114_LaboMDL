import React from 'react';
import ListGroup from 'react-bootstrap/ListGroup';
import { NavLink } from 'react-router-dom';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { faDoorOpen } from '@fortawesome/free-solid-svg-icons';
import { useLocation } from 'react-router-dom';
import Tooltip from 'react-bootstrap/Tooltip';
import OverlayTrigger from 'react-bootstrap/OverlayTrigger';
import { useAuth } from '../context/authContext';
import { Link } from 'react-router-dom';
import layout from '../lib/layout';

function Sidebar({ open, links }) {
  const location = useLocation();
  const { user } = useAuth();

  const localPath = `/${location.pathname.split('/')[1]}`;
  if (layout.layout(localPath) == 'alternate') return <></>;

  return (
    <nav className="sidebar p-2">
      {/* Logo */}
      <NavLink to="/">
        <div className="sidebar-logo">
          <img
            src="https://www.freelogodesign.org/file/app/client/thumb/93b27fc8-6653-43ea-a4c4-3f22893f93dd_200x200.png?1585111496240"
            alt="logo"
            height="53"
          />
        </div>
      </NavLink>
      {/* Menu */}
      <ListGroup variant="flush" className="sidebar-nav w-100 text-center">
        {links.map(({ name, path, icon, title }, i) => (
          <NavLink exact to={path} key={i}>
            <ListGroup.Item>
              <OverlayTrigger
                placement="right"
                overlay={<Tooltip>{title}</Tooltip>}
              >
                <Icon icon={icon} />
              </OverlayTrigger>
            </ListGroup.Item>
          </NavLink>
        ))}
        {user ? (
          <NavLink variant="danger" exact to="/logout" className="sidebar-exit">
            <ListGroup.Item>
              <Icon icon={faDoorOpen} />
            </ListGroup.Item>
          </NavLink>
        ) : (
          <></>
        )}
      </ListGroup>
    </nav>
  );
}

Sidebar.defaultProps = {
  links: [],
};

export default Sidebar;
