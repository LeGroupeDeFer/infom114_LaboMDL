import React, { useState } from 'react';
import ReactDOM from 'react-dom';
import AutoForm from '../../js/components/AutoForm';
import sinon from 'sinon';
import { mount, shallow } from 'enzyme';


describe('<AutoForm />', () => {

  let clock;
  let container;
  beforeEach(() => {
    clock = sinon.useFakeTimers();
    container = document.createElement('div');
    document.body.appendChild(container);
  });
  afterEach(() => {
    clock.restore();
    document.body.removeChild(container);
    container = null;
  });

  it('should render', () => {
    const onSubmit = () => new Promise(resolve => res);
    const wrapper = shallow(<AutoForm onSubmit={onSubmit} />);

    expect(wrapper.find('form')).toBeTruthy();
  });

  it('should render children', () => {
    const onSubmit = jest.fn(_ => new Promise(resolve => resolve(42)));

    const wrapper = mount(
      <AutoForm onSubmit={onSubmit}>
        <AutoForm.Submit>
          Submit
        </AutoForm.Submit>
      </AutoForm>
    );

    const button = wrapper.find('button');
    expect(wrapper.find('button')).toHaveLength(1);
  });

  it('should trigger callbacks', () => {
    const onSubmit = jest.fn(_ => new Promise(resolve => resolve(42)));

    const wrapper = mount(
      <AutoForm onSubmit={onSubmit}>
        <AutoForm.Submit>
          Submit
        </AutoForm.Submit>
      </AutoForm>
    );

    wrapper.find('button').first().simulate('submit');
    expect(onSubmit).toHaveBeenCalledTimes(1);
  });

  it.skip('should submit a data object', () => {
  });
});