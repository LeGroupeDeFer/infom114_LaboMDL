import React from 'react';
import { shallow, mount } from 'enzyme';

import Circle from '../../js/components/Circle';


describe('<Circle />', () => {

  it('should render', () => {
    const wrapper = shallow(<Circle />);
    expect(wrapper).toBeTruthy();
  });

  it('should render children', () => {
    const wrapper = mount(
      <Circle>
        <p>A circled text</p>
      </Circle>
    );

    expect(wrapper.contains(<p>A circled text</p>)).toEqual(true);
  });

  it('should respect circle classes', () => {
    const wrapper = shallow(<Circle />);

    expect(wrapper.find('.shape-circle')).toHaveLength(1);
    expect(wrapper.find('.shape-content')).toHaveLength(1);
  });

});