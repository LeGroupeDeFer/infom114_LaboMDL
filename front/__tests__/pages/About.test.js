import React from 'react';
import { shallow } from 'enzyme';

import About from '../../js/pages/About';

// Dummy test while the component is not implemented yet
describe('<About />', () => {

  it('should render', () => {
    const wrapper = shallow(<About />);
    expect(wrapper).toBeTruthy();
  });

});
