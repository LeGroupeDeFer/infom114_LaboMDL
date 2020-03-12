import { mount, shallow } from 'enzyme';
import React from 'react';
import Post from '../../js/components/Post';
import { preview } from '../../js/lib';



/* Will probably evolve as post is incomplete right now */
describe('<Post />', () => {

  it('should render', () => {
    const wrapper = shallow(<Post />);
    expect(wrapper).toBeTruthy();
  });

  it.skip('should render default props', () => {
    // TODO
  });

  it.skip('should render props', () => {
    // TODO
  });

});