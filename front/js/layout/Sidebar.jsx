import React from 'react';
import ListGroup from 'react-bootstrap/ListGroup';
import {Link, NavLink} from 'react-router-dom';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { useLocation } from 'react-router-dom';
import Tooltip from 'react-bootstrap/Tooltip';
import OverlayTrigger from 'react-bootstrap/OverlayTrigger'
import layout from '../lib/layout';

function Sidebar({ open, links }) {
  const location = useLocation();
  
  const localPath = `/${location.pathname.split('/')[1]}`;
  if (layout.layout(localPath) == 'alternate') return <></>;

  return (
    <nav className="sidebar p-2">
      {/* Logo */}
      <div className="sidebar-logo">
        <Link to="/">
          <img
            src="https://www.freelogodesign.org/file/app/client/thumb/93b27fc8-6653-43ea-a4c4-3f22893f93dd_200x200.png?1585111496240"
            alt="logo"
            height="53"
          />
        </Link>
      </div>
      {/* Menu */}
      <ListGroup variant="flush" className="sidebar-nav w-100 text-center">
        {links.map(({ name, path, icon, title }, i) => (
          <OverlayTrigger
            key={name}
            placement="right"
            positionTop={150}
            style={{ top: '20px !important' }}
            overlay={<Tooltip className="nav-tooltip">{title}</Tooltip>}
          >
            <NavLink exact to={path} key={i}>
              <ListGroup.Item>
                <Icon icon={icon} />
              </ListGroup.Item>
            </NavLink>
          </OverlayTrigger>
        ))}
      </ListGroup>
    </nav>
  );
}

Sidebar.defaultProps = {
  links: [],
};

export default Sidebar;
