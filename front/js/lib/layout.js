class LayoutError extends Error {}


const link = (name, path, icon, title, n) => ({ name, path, icon, title, n });

const layouts = {
  'base': ['/', '/about', '/settings', '/profile', '/notifications', '/admin'],
  'alternate': ['/login', '/register', '/activate', '/recover', '/restore']
};

const nobodyLinks = [
  link('stream', '/', "stream", 'Fil d\'actualité', 1),
  link('about', '/about', "info-circle", 'À propos', 5),
  link('settings', '/settings', "cogs", 'Paramètres', 4),
  link('profile', '/profile', "user-circle", 'Profil', 3),
];

const authenticatedLinks = [];
//  link('notifications', '/notifications', "bell", 'Notifications', 2)


const adminLinks = [
  link('admin', '/admin', "user-shield", 'Admin', 6)
];

function links(user = null, token = null) {
  let admin = false;
  if (token!=null) {
    const caps = token.cap;
    
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
}


export default {
  layout,
  links
};
