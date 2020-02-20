
export const bootstrapVariants = [
  'primary', 'outline-primary',
  'secondary', 'outline-secondary',
  'success', 'outline-success',
  'info', 'outline-info',
  'warning', 'outline-warning',
  'danger', 'outline-danger',
  'light', 'outline-light',
  'dark', 'outline-dark',
];

export const colorVariants = Object.freeze({
  'primary': '#55AB26'
});

export const capitalize = str =>
  `${str.charAt(0).toUpperCase()}${str.slice(1).toLowerCase()}`;

// Temporary code to justify Jest tests
export const sum = (a, b) => a + b;