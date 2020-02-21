import React, { forwardRef } from 'react';

const Circle = forwardRef(({ width, children, ...others }, ref) => (
  <div ref={ref} {...others}>
    <div
      className='shape-circle'
      style={{ width }}
    >
      <div className="shape-content">
        {children}
      </div>
    </div>
  </div>
));

Circle.defaultProps = {
  width: 'auto'
};

export default Circle;