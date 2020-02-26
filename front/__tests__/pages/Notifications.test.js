import React from 'react';
import { shallow } from 'enzyme';

import Notifications from '../../js/pages/Notifications';


// Dummy test while the component is not implemented yet
describe('<Notification />', () => {

  it('should render', () => {
    const wrapper = shallow(<Notifications />);
    expect(wrapper).toBeTruthy();
  });

});
