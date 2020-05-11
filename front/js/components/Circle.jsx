import React, { forwardRef } from 'react';
import clsx from "clsx";

const Circle = forwardRef(({ width, children, className, ...others }, ref) => (
  <div ref={ref} {...others}>
    <div
      className={clsx('shape-circle', className)}
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