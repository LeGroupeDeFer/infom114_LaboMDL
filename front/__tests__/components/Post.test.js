import { act, fireEvent, render } from '@testing-library/react';
import React from 'react';
import Post from '../../js/components/Post';
import { preview } from '../../js/lib';



/* Will probably evolve as post is incomplete right now */
describe('<Post />', () => {

  it('should render', () => {
    const { container } = render(<Post />);
    const post = container.querySelector('.post');
    expect(post).toBeDefined();
  });

  it.skip('should render default props', () => {
    // TODO
  });

  it.skip('should render props', () => {
    // TODO
  });

});