
/* ------------------------------ App consts ------------------------------- */

// bootstrapVariants :: Array<String>
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

// colorVariants :: Map<String, String>
export const colorVariants = Object.freeze({
  'primary': '#55AB26'
});

// breakpoints :: Map<String, Integer>
export const breakpoints = Object.freeze({
  'sm': 576, 'md': 768, 'lg': 992, 'xl': 1200
});


/* ------------------------------- I/O utils ------------------------------- */

// println :: (...Any) => None
/* istanbul ignore next */
export const println = console.log.bind(console);

// trace<T> :: T => T
export const trace = x => println(x) || x;


/* ------------------------------- DOM utils ------------------------------- */

// scrollbarWidth :: None => Integer
/* istanbul ignore next */
export const scrollbarWidth = () =>
  window.innerWidth - document.documentElement.clientWidth;


/* ----------------------------- String utils ------------------------------ */

// capitalize :: String => String
export const capitalize = str => (
  !str.length ? str : (
    str[0] == ' '
      ? ' ' + capitalize(str.slice(1))
      : `${str.charAt(0).toUpperCase()}${str.slice(1).toLowerCase()}`
  ));


/* ---------------------------- Function utils ----------------------------- */

// debounce<...Ts> :: (Callable<...Ts>, Integer?) => Callable<...Ts>
export function debounce(fn, ms = 250) {
  if (ms === 0)
    return fn;

  let timer;
  return _ => {
    clearTimeout(timer);
    timer = setTimeout(_ => {
      timer = null;
      fn.apply(this, arguments)
    }, ms);
  };
}
