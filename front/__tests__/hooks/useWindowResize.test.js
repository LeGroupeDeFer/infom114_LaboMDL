import React from 'react';
import { act } from 'react-dom/test-utils';
import ReactDOM from 'react-dom';
import sinon from 'sinon';

import useWindowResize from '../../js/hooks/useWindowResize';

const resizeWindow = (x, y) => {
  Object.assign(window, {
    innerWidth: x,
    innerHeight: y,
  });
  window.dispatchEvent(new Event('resize'));
};

describe('useWindowResize', () => {

  // Setup a clock as the useWindowResize is debounced
  let clock;
  let container;
  beforeEach(() => {
    clock = sinon.useFakeTimers();
    container = document.createElement('div');
    document.body.appendChild(container);
  });
  afterEach(() => {
    clock.restore();
    document.body.removeChild(container);
    container = null;
  });

  // We need to create a custom element to test the hook as hooks can't be used
  // in regular functions
  const Element = props => {
    const dimensions = useWindowResize(250);
    return (
      <div {...props} style={dimensions} />
    );
  };

  it('should return document values', () => {
    act(() => {
      ReactDOM.render(<Element className='test' />, container);
    });

    const target = document.querySelector('.test');
    expect(target.style.width).toBe('1024px');
    expect(target.style.height).toBe('768px');

    act(() => {
      resizeWindow(500, 500);
      clock.tick(500);
    });

    expect(target.style.width).toBe('500px');
    expect(target.style.height).toBe('500px');
  });

  it('should respect debounce value', () => {
    act(() => {
      ReactDOM.render(<Element className='test' />, container);
    });

    const target = document.querySelector('.test');
    const { width, height } = target.style;

    act(() => {
      // Shouldn't resize Element as it is debounced
      resizeWindow(500, 500);
    });

    expect(target.style.width).toBe(width);
    expect(target.style.height).toBe(height);
  });

});
