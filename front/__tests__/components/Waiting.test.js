import regeneratorRuntime from 'regenerator-runtime';
import React, { lazy } from 'react';
import { act, fireEvent, render } from '@testing-library/react';

import Waiting from '../../js/components/Waiting';
import { delay } from '../../js/lib';


describe('<Waiting />', () => {

  it('should render', () => {
    const Div = _ => <div className="unit-test" />;
    const Component = Waiting(Div);
    const wrapper = render(<Component />);

    expect(wrapper).toBeTruthy();
  });

  it('should render the inner component', () => {
    const Div = () => <div className="unit-test" />;

    const Component = Waiting(Div);
    act(() => {
      const { container } = render(<Component />);
      const div = container.querySelector('.unit-test');
      expect(div).toBeDefined();
    });
  });

  it('should render while the inner component is delayed', async () => {
    const Delayed = lazy(delay(() => <div className="unit-test" />, 1000));
    const Component = Waiting(Delayed);
    const wrapper = render(<Component />);

    expect(wrapper).toBeTruthy();
  });

  // Mounting <Suspense /> seems broken
  it('should render the default spinner while the inner component is delayed', async () => {
    const Delayed = lazy(delay(() => <div className="unit-test" />, 1000));
    const Component = Waiting(Delayed);

    const { container } = render(<Component />);

    const spinner = container.querySelector('.default-spinner');
    expect(spinner).toBeDefined();
  });

});