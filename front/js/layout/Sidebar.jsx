import React, { forwardRef } from 'react';
import { NavLink } from 'react-router-dom';
import ListGroup from 'react-bootstrap/ListGroup';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import Tooltip from 'react-bootstrap/Tooltip';
import OverlayTrigger from 'react-bootstrap/OverlayTrigger';
import { capitalize } from '../utils';
import clsx from 'clsx';

const Sidebar = forwardRef(({ open, links }, ref) =>
  <nav
    ref={ref}
    className={clsx('sidebar', open && 'sidebar-open', 'p-2')}
  >

    {/* Logo */}
    <div className="sidebar-logo">
      <img src="https://via.placeholder.com/50" alt="logo" />
    </div>

    {/* Menu */}
    <ListGroup variant='flush' className='sidebar-nav w-100 text-center'>
      {links.map(({ name, path, icon }, i) => (
        <NavLink exact to={path} key={i}>
          <ListGroup.Item>
            <OverlayTrigger
              trigger='hover'
              placement='right'
              delay={{ show: 250, hide: 250 }}
              overlay={p => <Tooltip {...p}>{capitalize(name)}</Tooltip>}
            >
              <Icon icon={icon} />
            </OverlayTrigger>
          </ListGroup.Item>
        </NavLink>
      ))}
    </ListGroup>

  </nav>
);

Sidebar.defaultProps = {
  links: []
};

export default Sidebar;