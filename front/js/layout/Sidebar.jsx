import React, { forwardRef } from 'react';
import ListGroup from 'react-bootstrap/ListGroup';
import { NavLink } from 'react-router-dom';
import { FontAwesomeIcon as Icon } from '@fortawesome/react-fontawesome';
import { useLocation } from 'react-router-dom';
import clsx from 'clsx';


const Sidebar = forwardRef(({ open, links }, ref) => {
  
  const location = useLocation();
  if (location.pathname == '/login')
    return <></>;

  return (
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
              <Icon icon={icon} />
            </ListGroup.Item>
          </NavLink>
        ))}
      </ListGroup>

    </nav>
  );
});

Sidebar.defaultProps = {
  links: []
};

export default Sidebar;