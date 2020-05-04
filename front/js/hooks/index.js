import { useEffect, useState } from 'react';
import { empty, zip, debounce, identity, Action, trace, equal } from '../lib';


export function usePositiveEffect(f, watched) {
  useEffect(() => watched.filter(identity).length ? f() : undefined);
}

export function useRequest(f, args, base = null) {

  const [error, setError] = useState(null);
  const [data, setData] = useState(base);
  const [promise, setPromise] = useState(null);
  const [localArgs, setLocalArgs] = useState([]);

  let differences = [];
  for (const [x, y] of zip(args || [], localArgs))
    if (!equal(x, y))
      differences.push(x);

  if (!empty(differences))
    setLocalArgs(args);

  useEffect(() => setPromise(f(...args)), [localArgs]);

  usePositiveEffect(() => {
    let isSubscribed = true;

    promise.then(
      retrievedData => isSubscribed ? setData(retrievedData || true) : undefined
    ).catch(
      retrievedError => isSubscribed ? setError(retrievedError || true) : undefined
    ).finally(() => setPromise(null));

    return () => isSubscribed = false;
  }, [promise]);

  return [error, data];
}

export function useAction(f) {
  const [action,] = useState(Action(f));
  const [error, data] = useRequest(() => action, []);
  return [action.onEvent, error, data];
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
