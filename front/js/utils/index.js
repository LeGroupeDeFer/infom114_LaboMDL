
export const bootstrapVariants = Object.freeze([
  'primary', 'outline-primary',
  'secondary', 'outline-secondary',
  'success', 'outline-success',
  'info', 'outline-info',
  'warning', 'outline-warning',
  'danger', 'outline-danger',
  'light', 'outline-light',
  'dark', 'outline-dark',
]);

export const colorVariants = Object.freeze({
  'primary': '#55AB26'
});

export const breakpoints = Object.freeze({
  'sm': 576, 'md': 768, 'lg': 992, 'xl': 1200
});

export const fakeLatency = 1500;

export const loremIpsum = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque volutpat vulputate nisl quis pulvinar. Praesent euismod magna metus, quis ultricies nunc sagittis in. Maecenas eleifend pulvinar nunc eu pulvinar. Fusce scelerisque, enim et bibendum aliquet, magna eros sodales eros, eu dictum neque mi a sapien. Aenean imperdiet cursus nisi in varius. Nam interdum imperdiet ante, sit amet sodales purus egestas sed. Proin sed felis tempus, viverra quam eu, convallis mi. In rhoncus velit lorem, interdum venenatis enim ornare at. Morbi mattis dignissim faucibus. Pellentesque pharetra ex non ante molestie rutrum. Pellentesque ullamcorper blandit turpis, eu molestie magna efficitur eget. Donec aliquet vulputate malesuada. Fusce porta nulla purus. Mauris purus ligula, elementum eu tincidunt in, consequat pretium ante. Duis eu leo eu arcu pharetra vestibulum.";

/* ------------------------------- I/O utils ------------------------------- */

export const println = console.log.bind(console);

export const trace = x => println(x) || x;

/* ------------------------------- DOM utils ------------------------------- */

export const scrollbarWidth = () =>
  window.innerWidth - document.documentElement.clientWidth;

/* ----------------------------- String utils ------------------------------ */

export const capitalize = str =>
  `${str.charAt(0).toUpperCase()}${str.slice(1).toLowerCase()}`;

/* ---------------------------- Function utils ----------------------------- */

export function debounce(fn, ms) {
  let timer;
  return _ => {
    clearTimeout(timer);
    timer = setTimeout(_ => {
      timer = null;
      fn.apply(this, arguments)
    }, ms);
  };
}

// Temporary code to justify Jest tests

export const sum = (a, b) => a + b;