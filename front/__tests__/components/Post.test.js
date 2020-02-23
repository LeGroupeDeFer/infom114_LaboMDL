import React from 'react';
import { shallow, mount } from 'enzyme';

import Post from '../../js/components/Post';
import { preview } from '../../js/utils';


/* Will probably evolve as post is incomplete right now */
describe('<Post />', () => {

  it('should render', () => {
    const wrapper = shallow(<Post />);
    expect(wrapper).toBeTruthy();
  });

  it('should render default props', () => {
    const wrapper = mount(<Post />);

    const titleWrapper = wrapper.find('div.card-title');
    expect(titleWrapper.text()).toEqual(Post.defaultProps.title);

    const textWrapper = wrapper.find('p.card-text');
    expect(textWrapper.text()).toEqual(
      preview(Post.defaultProps.text, Post.defaultProps.previewLength)
    );
  });

  it('should render props', () => {
    const wrapper = mount(
      <Post title='unit-test' text='Unit Testing' previewLength={3} />
    );

    const titleWrapper = wrapper.find('div.card-title');
    expect(titleWrapper.text()).toEqual('unit-test');

    const textWrapper = wrapper.find('p.card-text');
    expect(textWrapper.text()).toEqual(preview('Unit Testing', 3));
  });

});