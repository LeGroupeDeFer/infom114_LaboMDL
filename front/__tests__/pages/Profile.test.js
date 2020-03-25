import React from 'react';
import { act, fireEvent, render } from '@testing-library/react';
import Profile from '../../js/pages/Profile';


// Dummy test while the component is not implemented yet
describe('<Profile />', () => {

  it('should render', () => {
    const wrapper = render(<Profile />);
    expect(wrapper).toBeTruthy();
  });

});
