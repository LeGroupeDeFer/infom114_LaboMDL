import React, { forwardRef } from 'react';
import { bool, oneOf } from 'prop-types';


function Flexbox({ align, direction, justify, reverse, className, ...others }) {

  const cAlign = align ? `align-items-${align}` : '';
  const cDirection = `flex-${direction}${reverse ? '-reverse' : ''}`;
  const cJustify = justify ? `justify-content-${justify}` : '';

  const cssClass = `d-flex ${cAlign} ${cDirection} ${cJustify} ${className}`;

  return (
    <div className={cssClass} {...others} />
  );

}

Flexbox.defaultProps = {
  className: '',
  direction: 'row',
  reverse: false
};

Flexbox.propTypes = {
  align: oneOf(['start', 'end', 'center', 'baseline', 'stretch']),
  direction: oneOf(['row', 'column']),
  justify: oneOf(['start', 'end', 'center', 'around', 'between']),
  reverse: bool.isRequired
};


export default Flexbox;
