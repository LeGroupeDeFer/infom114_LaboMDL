import clsx from 'clsx';
import React from 'react';

function Image({ path, cover, ...others }) {

  const cls = clsx('image', cover && 'image-cover');
  return (
    <div className={cls} style={{ backgroundImage: `url(${path})` }} />
  );

}

Image.defaultProps = {
  cover: false
};

export default Image;