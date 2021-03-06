import { useEffect, useState } from 'react';
import { debounce } from '../lib';


export default function useWindowResize(debounceTimer = 250) {

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
