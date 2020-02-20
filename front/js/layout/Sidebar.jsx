import React from 'react';
import { NavLink } from 'react-router-dom';
import ListGroup from 'react-bootstrap/ListGroup';
import { capitalize } from '../utils';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import Tooltip from 'react-bootstrap/Tooltip';
import OverlayTrigger from 'react-bootstrap/OverlayTrigger';
import clsx from 'clsx';

export default function Sidebar(props) {

  const hamburgerStyle = {
    background: 'white',
    border: 'none'
  };

  const imageWidth = window.innerWidth < 768 ? '160px' : '240px';
  const firstname = capitalize(props.user.firstname);
  const lastname = capitalize(props.user.lastname);

  return (
    <nav className={clsx('sidebar', props.open && 'sidebar-open', 'p-2')}>

      {/* Hamburger button */}
      <div className="sidebar-logo">
        <img src="https://via.placeholder.com/50" alt="logo" />
      </div>

      {/* Actual menu */}
      <ListGroup variant='flush' className='sidebar-nav w-100 text-center'>
        {props.links.map(({ name, path, icon }, i) => (
          <NavLink exact to={path} key={i}>
            <OverlayTrigger
              placement='right'
              delay={{ show: 250, hide: 400 }}
              overlay={props => <Tooltip {...props}>{name}</Tooltip>}
            >
              <ListGroup.Item>
                <Icon icon={icon} />
              </ListGroup.Item>
            </OverlayTrigger>
          </NavLink>
        ))}
      </ListGroup>

    </nav>
  );

}

Sidebar.defaultProps = {
  user: {
    firstname: 'John',
    lastname: 'Doe',
    picture: 'https://via.placeholder.com/240'
  },
  links: []
};