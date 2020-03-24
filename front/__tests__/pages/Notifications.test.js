import React from 'react';
import { act, fireEvent, render } from '@testing-library/react';

import Notifications from '../../js/pages/Notifications';


// Dummy test while the component is not implemented yet
describe('<Notification />', () => {

  it('should render', () => {
    const wrapper = render(<Notifications />);
    expect(wrapper).toBeTruthy();
  });

});
