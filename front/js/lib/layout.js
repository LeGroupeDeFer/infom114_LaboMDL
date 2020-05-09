import {
  faUserCircle,
  faCogs,
  faInfoCircle,
  faStream,
  faBell,
  faUserShield,
  
} from '@fortawesome/free-solid-svg-icons';


class LayoutError extends Error {}


const link = (name, path, icon, title, n) => ({ name, path, icon, title, n });

const layouts = {
  'base': ['/', '/about', '/settings', '/profile', '/notifications', '/admin'],
  'alternate': ['/login', '/register', '/activate', '/recover', '/restore']
};

const nobodyLinks = [
  link('stream', '/', faStream, 'Fil d\'actualité', 1),
  link('about', '/about', faInfoCircle, 'À propos', 2),
  link('settings', '/settings', faCogs, 'Paramètres', 3)
];

const authenticatedLinks = [
  link('profile', '/profile', faUserCircle, 'Profil', 3),
  link('notifications', '/notifications', faBell, 'Notifications', 4)
];

const adminLinks = [
  link('admin', '/admin', faUserShield, 'Admin', 5)
];

function links(user = null, token = null) {
  let admin = false;
  if (token!=null) {
    const caps = token.cap;
    console.log(token.cap);
    
    for(let i = 0; i < caps.length; i++) {
        if (caps[i].name == 'admin:access') {
            admin = true;
            break;
        }
    }
  }  

  return [ ...nobodyLinks, ...(user ? authenticatedLinks : []), ...(admin ? adminLinks : []) ].sort(
    (a, b) => a.n - b.n
  );
}

function layout (location) {
  const selected = Object.keys(layouts).filter(
    key => layouts[key].includes(location)
  );
  if (!selected.length)
    return 'base';
  return selected[0];
};


export default {
  layout,
  links
};
