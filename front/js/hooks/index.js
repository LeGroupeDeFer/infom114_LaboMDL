import { useEffect, useState } from 'react';
import { debounce, identity, Action } from '../lib';
import isEqual from 'lodash/isEqual';

export function usePositiveEffect(f, watched) {
  useEffect(() => watched.filter(identity).length ? f() : undefined);
}

/**
 * @callback Equal
 * @returns boolean
 *
 * An arbitrary equality check function
 */

/**
 * @callback Effect
 * @returns { CallableFunction | void } The cleanup function
 *
 * A potentially IO-bound, stateful side-effect.
 */

/**
 * Same behaviour as useEffect, except that the effect triggers only whenever
 * the `watched` argument change **as described by the `eq` function argument**.
 *
 * @param {Effect} f The effect
 * @param {Array<any>} watched The effect function arguments
 * @param {Equal} [eq] The equality comparison function, defaults to lodash `isEqual`
 */
export function useDiffEffect(f, watched, eq = isEqual) {
  let [local, setLocal] = useState(watched);
  if (!eq(watched, local))
    setLocal(watched);
  useEffect(f, [local]);
}

export const subscribed = (promise, onResolve, onReject) => () => {
  let isSubscribed = true;
  promise
    .then(data => isSubscribed ? onResolve(data) : undefined)
    .catch(error => isSubscribed ? onReject(error) : undefined);
  return () => isSubscribed = false;
};

export function useRequest(f, args, base = null, eq = isEqual) {
  const [error, setError] = useState(null);
  const [data, setData] = useState(base);
  const [promise, setPromise] = useState(null);

  useDiffEffect(() => setPromise(f(...args)), [args], eq);

  usePositiveEffect(
    subscribed(promise, d => setData(d || true), e => setError(e || true)),
    [promise]
  );

  return [error, data];
}

export function useAction(f) {
  const [action,] = useState(Action(f));
  const [state, setState] = useState([action && action.onEvent, null, null])

  useEffect(() => subscribed(
    action,
    data => setState([action && action.onEvent, null, data || true]),
    error => setState([action && action.onEvent, error || true, null]),
  ), [action.watch]);

  return state;
}

export function useWindowResize(debounceTimer = 250) {
  const [height, setHeight] = useState(window.innerHeight);
  const [width, setWidth] = useState(window.innerWidth);

  const handleResize = debounce(() => {
    setHeight(window.innerHeight);
    setWidth(window.innerWidth);
  }, debounceTimer);

  useEffect(() => {
    window.addEventListener('resize', handleResize);
    return (() => window.removeEventListener('resize', handleResize));
  });

  return { width, height };
}

export function useEffectQueue() {
  const [queue, setQueue] = useState([]);

  usePositiveEffect(() => {
    const cleanup = queue.map(([p, res, rej]) => subscribed(p, res, rej));
    setQueue([]);
    return () => cleanup.forEach(cleanup => cleanup());
  }, queue);

  return effect => setQueue(oldQueue => ([...oldQueue, effect]));
}
