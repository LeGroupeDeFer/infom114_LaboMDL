import React from 'react';
import { shallow } from 'enzyme';

import Settings from '../../js/pages/Settings';


// Dummy test while the component is not implemented yet
describe('<Settings />', () => {

  it('should render', () => {
    const wrapper = shallow(<Settings />);
    expect(wrapper).toBeTruthy();
  });

});
