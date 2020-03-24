import React from 'react';
import { act, fireEvent, render } from '@testing-library/react';

import { AuthProvider } from '../../js/context/authContext';
import Login from '../../js/pages/Login';

jest.mock('react-router-dom', () => ({
  useHistory: () => ({
    push: jest.fn(),
    replace: jest.fn()
  }),
  Link: () => <div />
}));

// Dummy test while the component is not implemented yet
describe('<Login />', () => {

  it('should render', () => {
    const wrapper = render(
      <AuthProvider>
        <Login />
      </AuthProvider>
    );
    expect(wrapper).toBeTruthy();
  });

});
