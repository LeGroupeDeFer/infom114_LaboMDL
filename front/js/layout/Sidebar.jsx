import React, { forwardRef } from 'react';
import { NavLink } from 'react-router-dom';
import ListGroup from 'react-bootstrap/ListGroup';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import Tooltip from 'react-bootstrap/Tooltip';
import OverlayTrigger from 'react-bootstrap/OverlayTrigger';
import clsx from 'clsx';

const Sidebar = forwardRef((props, ref) =>
  <nav
    ref={ref}
    className={clsx('sidebar', props.open && 'sidebar-open', 'p-2')}
  >

    {/* Hamburger button */}
    <div className="sidebar-logo">
      <img src="https://via.placeholder.com/50" alt="logo" />
    </div>

    {/* Actual menu */}
    <ListGroup variant='flush' className='sidebar-nav w-100 text-center'>
      {props.links.map(({ name, path, icon }, i) => (
        <NavLink exact to={path} key={i}>
          <ListGroup.Item>
            <OverlayTrigger
              trigger='hover'
              placement='right'
              delay={{ show: 250, hide: 250 }}
              overlay={props => <Tooltip {...props}>{name}</Tooltip>}
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
  user: {
    firstname: 'John',
    lastname: 'Doe',
    picture: 'https://via.placeholder.com/240'
  },
  links: []
};

export default Sidebar;