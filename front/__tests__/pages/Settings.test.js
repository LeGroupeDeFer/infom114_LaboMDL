import React from 'react';
import { act, fireEvent, render } from '@testing-library/react';

import Settings from '../../js/pages/Settings';


// Dummy test while the component is not implemented yet
describe('<Settings />', () => {

  it('should render', () => {
    const wrapper = render(<Settings />);
    expect(wrapper).toBeTruthy();
  });

});
