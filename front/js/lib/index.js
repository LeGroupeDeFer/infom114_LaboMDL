import { lazy } from 'react';

const identity = x => x;

/* ------------------------------ App consts ------------------------------- */

// bootstrapVariants :: Array<String>
const bootstrapVariants = Object.freeze([
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
const colorVariants = Object.freeze({
  'primary': '#55AB26'
});

// breakpoints :: Map<String, Integer>
/**
 * @memberof lib
 */
const breakpoints = Object.freeze({
  'sm': 576, 'md': 768, 'lg': 992, 'xl': 1200
});


/* ------------------------------- I/O utils ------------------------------- */

/* istanbul ignore next */
/**
 * @memberof lib
 */
const println = console.log.bind(console);

/* istanbul ignore next */
/**
 * @memberof lib
 */
const printerr = console.error.bind(console);

/**
 * Debugging utility. Outputs the given value to console.err and returns the value.
 * @memberof lib
 *
 * @param { any } thing The traced object.
 * @return { any } The given value
 */
const trace = x => printerr(x) || x;


/* ------------------------------- DOM utils ------------------------------- */

/* istanbul ignore next */
/**
 * Query the document scrollbar width.
 * @memberof lib
 *
 * @returns { int } The scrollbar width.
 */
const scrollbarWidth = () =>
  window.innerWidth - document.documentElement.clientWidth;

/* istanbul ignore next */
/**
 * [Element.querySelector]{@link https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelector} shortcut.
 * @memberof lib
 * @param { string } selector The CSS selector.
 * @param { Element } [parent=document]  The DOM Element on which to execute the query.
 */
const query = (selector, parent = document) =>
  parent.querySelector(selector);

/* istanbul ignore next */
/**
 * [Element.querySelectorAll]{@link https://developer.mozilla.org/en-US/docs/Web/API/Element/querySelectorAll} shortcut.
 * @memberof lib
 * @param { string } selector The CSS selector.
 * @param { Element } [parent=document]  The DOM Element on which to execute the query.
 */
const queryAll = (selector, parent = document) =>
  parent.querySelectorAll(selector);

/* ----------------------------- String utils ------------------------------ */

/**
 * Returns a copy of the given string with the first letter uppercased, if any.
 * @memberof lib
 *
 * @param { string } str The string to capitalize.
 * @returns { string } The capitalized string.
 */
const capitalize = str => (
  !str.length ? str : (
    str[0] == ' '
      ? ' ' + capitalize(str.slice(1))
      : `${str.charAt(0).toUpperCase()}${str.slice(1).toLowerCase()}`
  ));

/**
 * Trims the given string to max `length` characters and suffix the result with three ellipsis. If the given string was shorther than `length`, it is returned as is.
 * @memberof lib
 *
 * @param { string } text The string to trim.
 * @param { number } [length=200] The max length.
 * @returns { string } The trimmed string
 */
const preview = (text, length = 200) =>
  text.length > length ? `${text.slice(0, length)}...` : text;

/* ---------------------------- Function utils ----------------------------- */

// debounce<...Ts> :: (Callable<...Ts>, Integer?) => Callable<...Ts>
/**
 * @memberof lib
 *
 * @param { function } fn
 * @param { int } [ms=250]
 * @returns { function }
 */
function debounce(fn, ms = 250) {
  if (ms <= 0)
    return fn;

  let timer;
  return function () {
    clearTimeout(timer);
    timer = setTimeout(() => {
      timer = null;
      fn.apply(this, arguments)
    }, ms);
  };
}

/**
 * @memberof lib
 *
 * @param { function } fn
 * @param { int } [ms=250]
 * @returns { function }
 */
function delay(fn, ms = 250) {
  return ms === 0 ? fn : (() => new Promise((resolve, _) => setTimeout(
    () => resolve(fn.apply(this, arguments)),
    ms
  )));
}

/**
 * @memberof lib.delay
 */
delay.lazy = (fn, ms = 250) => lazy(() => delay(fn, ms));

/* ------------------------------- Map utils ------------------------------- */

// TODO - Transform tail recursion in stack-based logic, js does not support tail recursion
function recurse(thing, kfn = identity, vfn = identity) {
  if ([null, undefined].includes(thing))
    return thing;

  if (Array.isArray(thing))
    return thing.map(x => recurse(x, kfn, vfn));

  if (thing.constructor === Object)
    return Object.keys(thing).reduce(
      (acc, key) => ({...acc, [kfn(key)]: recurse(thing[key], kfn, vfn) }),
      {}
    );

  return vfn(thing);
}

const _camel = s => s.replace(
  /([-_][a-z])/ig,
  (match) => match.toUpperCase().replace('-', '').replace('_', '')
);

const camel = thing => recurse(thing, _camel);

const _snake = s => s.replace(
  /\.?([A-Z]+)/g,
  (_, match) => `_${match.toLowerCase()}`
).replace(/^_/, '');

const snake = thing => recurse(thing, _snake);

/* -------------------------------- Exports -------------------------------- */


import api from './api';
import * as dev from './dev';
import * as validators from './validators';

/** @namespace lib */
export {
  identity,

  api,
  dev,
  validators,

  colorVariants,
  breakpoints,

  println,
  printerr,
  trace,

  scrollbarWidth,
  capitalize,
  preview,
  debounce,
  delay,

  recurse,
  camel,
  snake
};
