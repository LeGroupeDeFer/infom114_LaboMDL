import React from 'react';
import { act, fireEvent, render } from '@testing-library/react';
import Stream from '../../js/pages/Stream';


// Dummy test while the component is not implemented yet
describe.skip('<Stream />', () => {

  it('should render', () => {
    const wrapper = render(<Stream />);
    expect(wrapper).toBeTruthy();
  });

});
