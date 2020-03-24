import React from 'react';
import { act, fireEvent, render } from '@testing-library/react';

import About from '../../js/pages/About';

// Dummy test while the component is not implemented yet
describe('<About />', () => {

  it('should render', () => {
    const wrapper = render(<About />);
    expect(wrapper).toBeTruthy();
  });

});
