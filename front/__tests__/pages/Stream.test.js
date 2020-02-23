import React from 'react';
import { shallow } from 'enzyme';

import Stream from '../../js/pages/Stream';


// Dummy test while the component is not implemented yet
describe('<Stream />', () => {

  it('should render', () => {
    const wrapper = shallow(<Stream />);
    expect(wrapper).toBeTruthy();
  });

});
