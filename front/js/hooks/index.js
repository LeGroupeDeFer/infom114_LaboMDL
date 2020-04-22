import { useEffect, useState } from 'react';
import { debounce, identity, Action, trace } from '../lib';


export function usePositiveEffect(f, watched) {
  useEffect(() => {
    if (watched.filter(identity).length)
      f();
  });
}

export function useRequest(f, args) {
  const [error, setError] = useState(null);
  const [data, setData] = useState(null);
  const [promise, setPromise] = useState(null);

  useEffect(() => setPromise(f(...args)), []);

  usePositiveEffect(() => {
    let isSubscribed = true;

    promise.then(
      retreivedData => isSubscribed ? setData(retreivedData || true) : undefined
    ).catch(
      retreivedError => isSubscribed ? setError(retreivedError || true) : undefined
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
