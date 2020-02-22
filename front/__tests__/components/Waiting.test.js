import React from 'react';
import { shallow, mount } from 'enzyme'

import Waiting from '../../js/components/Waiting';
import { delay } from '../../js/utils';


describe('<Waiting />', () => {

  it('should render', () => {
    const Div = _ => <div className="unit-test" />;
    const Component = Waiting(Div);
    const wrapper = shallow(<Component />);

    expect(wrapper).toBeTruthy();
  });

  it('should render the inner component', () => {
    const Div = _ => <div className="unit-test" />;
    const Component = Waiting(Div);
    const wrapper = mount(<Component />);

    expect(wrapper.contains(<div className="unit-test" />)).toEqual(true);
  });

  it('should render while the inner component is delayed', () => {
    const Delayed = delay.lazy(() => <div className="delayed" />, 1000);
    const Component = Waiting(Delayed);
    const wrapper = shallow(<Component />);

    expect(wrapper).toBeTruthy();
  });

  // Mounting <Suspense /> seems broken
  it.skip('should render the default spinner while the inner component is delayed', () => {
    const Delayed = delay.lazy(() => <div className="delayed" />, 1000);
    const Component = Waiting(Delayed);
    const wrapper = mount(<Component />);

    expect(wrapper.contains(<Waiting.DefaultSpinner />)).toBeTruthy();
  });

});