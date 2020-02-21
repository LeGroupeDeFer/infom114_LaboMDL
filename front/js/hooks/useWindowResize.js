import { useEffect, useState } from 'react';
import { debounce } from '../utils';


export default function useWindowResize(debounceTimer = 250) {

  const [height, setHeight] = useState(window.innerHeight);
  const [width, setWidth] = useState(window.innerWidth);

  const handleResize = debounce(function () {
    setHeight(window.innerHeight);
    setWidth(window.innerWidth);
  }, debounceTimer);

  useEffect(() => {
    window.addEventListener('resize', handleResize);
    return (_ => window.removeEventListener('resize', handleResize));
  });

  return { width, height };
}