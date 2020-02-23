import React from 'react';
import { shallow } from 'enzyme';

import Profile from '../../js/pages/Profile';


// Dummy test while the component is not implemented yet
describe('<Profile />', () => {

  it('should render', () => {
    const wrapper = shallow(<Profile />);
    expect(wrapper).toBeTruthy();
  });

});
