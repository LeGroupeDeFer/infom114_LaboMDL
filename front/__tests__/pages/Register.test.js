import React from 'react';
import { act, fireEvent, render } from '@testing-library/react';

import Register from '../../js/pages/Register';
import { AuthProvider } from '../../js/context/authContext';

jest.mock('react-router-dom', () => ({
  useHistory: () => ({
    push: jest.fn(),
    replace: jest.fn()
  }),
  Link: () => <div />
}));


// Dummy test while the component is not implemented yet
describe('<Register />', () => {

  it('should render', () => {
    const wrapper = render(
      <AuthProvider>
        <Register />
      </AuthProvider>
    );
    expect(wrapper).toBeTruthy();
  });

});
