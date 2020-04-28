import {
  faUserCircle,
  faCogs,
  faInfoCircle,
  faStream,
  faBell,
  faPencilAlt
} from '@fortawesome/free-solid-svg-icons';


class LayoutError extends Error {}


const link = (name, path, icon, title, n) => ({ name, path, icon, title, n });

const layouts = {
  'base': ['/', '/about', '/settings', '/profile', '/notifications'],
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


function links(user = null) {
  return [ ...nobodyLinks, ...(user ? authenticatedLinks : []) ].sort(
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
