import React from 'react';
import { act, fireEvent, render } from '@testing-library/react';

import Logout from '../../js/pages/Logout';
import { AuthProvider } from '../../js/context/authContext';

jest.mock('react-router-dom', () => ({
  useHistory: () => ({
    push: jest.fn(),
    replace: jest.fn()
  }),
  Link: () => <div />
}));


// Dummy test while the component is not implemented yet
describe('<Logout />', () => {

  it('should render', () => {
    const wrapper = render(
      <AuthProvider>
        <Logout />
      </AuthProvider>
    );
    expect(wrapper).toBeTruthy();
  });

});
