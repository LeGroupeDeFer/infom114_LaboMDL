import { lazy } from 'react';

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

// printerr :: (...Any) => None
/* istanbul ignore next */
export const printerr = console.error.bind(console);

// trace<T> :: T => T
export const trace = x => printerr(x) || x;


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

// preview :: (String, Integer?) => String
export const preview = (text, length = 200) =>
  text.length > length ? `${text.slice(0, length)}...` : text;

/* ---------------------------- Function utils ----------------------------- */

// debounce<...Ts> :: (Callable<...Ts>, Integer?) => Callable<...Ts>
export function debounce(fn, ms = 250) {
  if (ms === 0)
    return fn;

  let timer;
  return () => {
    clearTimeout(timer);
    timer = setTimeout(() => {
      timer = null;
      fn.apply(this, arguments)
    }, ms);
  };
}

// delay<...Ts> :: (Callable<...Ts>, Integer?) => Callable<...Ts>
export function delay(fn, ms = 250) {
  return ms === 0 ? fn : (() => new Promise((resolve, _) => setTimeout(
    () => resolve(fn.apply(this, arguments)),
    ms
  )));
}

delay.lazy = (fn, ms = 250) => lazy(() => delay(fn, ms));