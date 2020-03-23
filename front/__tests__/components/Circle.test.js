import React from 'react';
import { act, fireEvent, render } from '@testing-library/react';
import Circle from '../../js/components/Circle';


describe('<Circle />', () => {

  it('should render', () => {
    const { container } = render(<Circle />);
    const circle = container.querySelector('.shape-circle');
    expect(circle).toBeDefined();
  });

  it('should render children', () => {
    const { container, getByText } = render(
      <Circle>
        <p>A circled text</p>
      </Circle>
    );

    const paragraph = getByText (/A circled text/);
    expect(paragraph).toBeDefined();
  });

  it('should respect circle classes', () => {

    const { container }= render(<Circle />);

    expect(container.querySelector('.shape-circle')).toBeDefined();
    expect(container.querySelector('.shape-content')).toBeDefined();
  });

});