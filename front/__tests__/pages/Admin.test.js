import React from 'react';
import { shallow } from 'enzyme';

import Admin from '../../js/pages/Admin';

// Dummy test while the component is not implemented yet
describe('<Admin />', () => {

  it('should render', () => {
    const wrapper = shallow(<Admin />);
    expect(wrapper).toBeTruthy();
  });

});
