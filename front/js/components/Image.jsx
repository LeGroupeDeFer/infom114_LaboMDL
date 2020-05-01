import clsx from 'clsx';
import React from 'react';

function Image({ path, cover, width, height, float, className, ...others }) {

  const cls = clsx(className, 'image', cover && 'image-cover');
  return (
    <div
      className={cls}
      style={{ backgroundImage: `url(${path})`, width, height, float }}
    />
  );

}

Image.defaultProps = {
  cover: false,
  width: '100%',
  height: '100%',
  float: 'unset'
};

export default Image;
