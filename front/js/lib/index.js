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

const ORDER = Object.freeze({
  RANK: Object.freeze({ DESC: 'high_rank', ASC: 'low_rank' }),
  SCORE: Object.freeze({ DESC: 'top', ASC: 'low' }),
  AGE: Object.freeze({ DESC: 'new', ASC: 'old' }),
});

const orders = Object.values(ORDER);
const orderOf = key =>
  head(Object.keys(ORDER).filter(k => k !== key).map(k => ORDER[k]));

const KIND = Object.freeze({
  ALL:  { label: 'Actualité', labelSingular: 'Actualité',   key: 'all',   value: 'all',   icon: "globe-europe"  },
  INFO: { label: 'Infos',     labelSingular: 'Information', key: 'info',  value: 'info',  icon: "info"          },
  IDEA: { label: 'Idées',     labelSingular: 'Idée',        key: 'idea',  value:'idea',   icon: "lightbulb"     },
  POLL: { label: 'Sondages',  labelSingular: 'Sondage',     key: 'poll',  value: 'poll',  icon: "balance-scale" },
});

const kinds = Object.values(KIND);
const kindOf = key =>
  head(Object.keys(KIND).map(k => KIND[k]).filter(k => k.key === key));

const VOTE = Object.freeze({
  DOWN: -1,
  NONE: 0,
  UP: 1,
});

const voteOf =
    v => head(Object.keys(VOTE).map(k => VOTE[k]).filter(vote => vote === v));

const WATCH_EVENT_FSM = Object.freeze([
  [false, true, false, false, false, false],  // Void
  [false, false, true, true, false, false],   // Submit
  [false, false, false, false, true, true],   // Accept
  [false, false, false, false, false, false], // Refuse
  [false, false, false, false, false, true],  // Progress - TODO progress -> progress
  [false, false, false, false, false, false], // Over
]);

const WATCH_EVENT = Object.freeze([
  { event: 0,                                                                           },
  { event: 1, doneLabel: 'Suivi',      actionLabel: 'Suivre',     icon: 'envelope-open' },
  { event: 2, doneLabel: 'Acceptée',   actionLabel: 'Accepter',   icon: 'check-circle'  },
  { event: 3, doneLabel: 'Déclinée',   actionLabel: 'Décliner',   icon: 'stop-circle'   },
  { event: 4, doneLabel: 'En progrès', actionLabel: 'Progresser', icon: 'tasks'         },
  { event: 5, doneLabel: 'Terminée',   actionLabel: 'Terminer',   icon: 'genderless'    }
]);

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
 * @param { any } x The traced object.
 * @return { any } The given value
 */
const trace = x => printerr(x) || x;

let _id = 0;
function Action(f) {
  const eventReference = `action${_id++}`;

  // Make a reference to the event source
  const detail = { source: null };

  // Create a fake event
  const handle = new CustomEvent(eventReference, { detail });

  // Withing the resulting promise, resolve when the fake event is triggered
  const promise = new Promise((resolve, _) => {
    document.addEventListener(
      eventReference, e => resolve((f||identity)(e.detail.source))
    );
  });

  // Give a handle to the caller
  promise.onEvent = e => {
    detail.source = e;
    document.dispatchEvent(handle);
    promise.watch = !promise.watch;
  };

  promise.watch = false;

  return promise;
}

Action.bind(Action);

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
 * Prevent default comportement from happening when using <a> tag with href='#'
 * @memberof lib
 * 
 * @param { event } e 
 * @param { function } fn 
 */
const prevent = (e, fn) => e.preventDefault() || e.stopPropagation() || fn();


/**
 * @memberof lib.delay
 */
delay.lazy = (fn, ms = 250) => lazy(() => delay(fn, ms));

/* ------------------------------- Map utils ------------------------------- */

// TODO - Transform tail recursion to stack-based logic, js does not support
// tail recursion
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

/* ----------------------------- Control flow ------------------------------ */

function iff(condition, value) {
  return condition ? value : undefined;
}

function tee(f, g) {
  return function(...params) {
    f(...params);
    return g(...params);
  }
}

const subscribed = (subscription, f) => subscription ? f() : undefined;

/* ----------------------------- Object utils ------------------------------ */

const defined = thing => thing !== undefined && thing !== null;

const truthy = thing => (thing instanceof Array) ? thing.length : thing;

const update = (o, k, v) => ({ ...o, [k]: v });

const clean = (o, hard= false) => Object.keys(o).reduce(
  (a, k) => (hard ? truthy : defined)(o[k]) ? ({ ...a, [k]: o[k] }) : a, {}
);

function aggregate(o, key, props) {
  const aggregation = {
    [key]: props.reduce((a, k) => ({ ...a, [k]: o[k] }), {})
  };
  const others = Object.keys(o)
    .filter(k => !props.includes(k))
    .reduce((a, k) => ({ ...a, [k]: o[k] }), {});

  return clean(Object.assign({}, others, aggregation));
}

function emptyObject(obj) {
  for (const key in obj)
    if(obj.hasOwnProperty(key))
      return false;
  return true;
}

/* ------------------------------ Array utils ------------------------------ */

const empty = xs => (xs instanceof Array) && xs.length === 0;

const head = xs => xs.length ? xs[0] : null;

const last = xs => (xs && xs.length) ? xs[xs.length - 1] : null;

const zip = (...xs) =>
  xs.length && xs[0].map((_, i) => xs.map(e => e[i])) || [];

/* -------------------------------- Exports -------------------------------- */


import api from './api';
import layout from './layout';
import * as dev from './dev';
import * as validators from './validators';

/** @namespace lib */
export {
  identity,

  api,
  layout,
  dev,
  validators,

  colorVariants,
  breakpoints,
  ORDER,
  orders,
  orderOf,
  KIND,
  kinds,
  kindOf,
  VOTE,
  voteOf,
  WATCH_EVENT,
  WATCH_EVENT_FSM,

  println,
  printerr,
  trace,
  Action,

  scrollbarWidth,
  capitalize,
  preview,

  debounce,
  delay,
  prevent,

  recurse,
  camel,
  snake,

  defined,
  update,
  clean,

  iff,
  tee,

  aggregate,
  emptyObject,

  empty,
  head,
  last,
  zip
};
