
// isUnamurEmail :: str => bool
export function isUnamurEmail(email) {
  return /@(student\.)?unamur\.be\s*$/.test(email);
}

// isValidPassword :: str => bool
export function isValidPassword(password) {
  return password.length > 3;
}
