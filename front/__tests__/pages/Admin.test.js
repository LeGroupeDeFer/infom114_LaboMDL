import React from 'react';
import { act, fireEvent, render } from '@testing-library/react';

import Admin from '../../js/pages/Admin';

// Dummy test while the component is not implemented yet
describe('<Admin />', () => {

  it('should render', () => {
    const wrapper = render(<Admin />);
    expect(wrapper).toBeTruthy();
  });

});
