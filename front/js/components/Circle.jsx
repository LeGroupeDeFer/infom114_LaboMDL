import React from 'react';
import clsx from 'clsx';

export default function Circle(props) {

  const { width, ...parentProps } = props;

  return (
    <div {...parentProps}>
      <div
        className='shape-circle'
        style={{ width }}
      >
        <div className="shape-content">
          {props.children}
        </div>
      </div>
    </div>
  );
}

Circle.defaultProps = {
  'width': 'auto'
};