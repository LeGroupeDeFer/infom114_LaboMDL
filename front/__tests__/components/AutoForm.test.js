import regeneratorRuntime from "regenerator-runtime";
import { act, fireEvent, render } from '@testing-library/react';
import { wait } from '@testing-library/dom';
import React from 'react';
import AutoForm from '../../js/components/AutoForm';
import { FormProvider, useForm } from '../../js/components/AutoForm/formContext';


class ErrorBoundary extends React.Component {
  constructor(props) {
    super(props);
    this.state = { hasError: false };
  }

  static getDerivedStateFromError(error) {
    return { hasError: true };
  }

  componentDidCatch(error, errorInfo) {
  }

  render() {
    if (this.state.hasError) {
      return <h1 id="test-error">Something went wrong.</h1>;
    }

    return this.props.children;
  }

}


function DebugFormProvider() {
  const { validity, error } = useForm();
  return (
    <>
      <div className="test-validity">{validity.toString()}</div>
      <div className="test-error">{error.toString()}</div>
    </>
  );
}


describe('<AutoForm.Control />', () => {

  it('should not render without a form context', () => {

    const spy = jest.spyOn(console, 'error');
    spy.mockImplementation(() => { });

    const { getByText } = render(
      <ErrorBoundary>
        <AutoForm.Control name="foo" />
      </ErrorBoundary>
    );

    expect(getByText('Something went wrong.')).toBeDefined();
    spy.mockRestore();
  });

  it('should render with a form context', () => {
    const { container } = render(
      <ErrorBoundary>
        <FormProvider onSubmit={() => null}>
          <AutoForm.Control name="foo" />
        </FormProvider>
      </ErrorBoundary>
    );

    expect(container.querySelector("#test-error")).toBeFalsy();
  });

  it('should provide a correct base validator', () => {
    const optionalValidator = AutoForm.Control.defaultValidator(null, true);
    const requiredValidator = AutoForm.Control.defaultValidator(null, false);

    expect(optionalValidator('')).toBeTruthy();
    expect(requiredValidator('')).toBeFalsy();
    expect(requiredValidator(42)).toBeTruthy();
  });

  it('should not fallback to base validator when a validator is provided', () => {
    const validator = x => x > 41 && x < 43;
    const optionalValidator = AutoForm.Control.defaultValidator(validator, true);
    const requiredValidator = AutoForm.Control.defaultValidator(validator, true);

    expect(optionalValidator).toBe(validator);
    expect(requiredValidator).toBe(validator);
  });

  it('should be valid when optional', () => {
    const callback = jest.fn();
    const { container } = render(
      <FormProvider onSubmit={callback}>
        <AutoForm.Control name="foo" optional />
        <DebugFormProvider />
      </FormProvider>
    );

    const validity = container.querySelector('.test-validity');
    expect(validity.textContent).toBe('true');
  });

  it('should be invalid when required', () => {
    const callback = jest.fn();
    const { container } = render(
      <FormProvider onSubmit={callback}>
        <AutoForm.Control name="foo" />
        <DebugFormProvider />
      </FormProvider>
    );

    const validity = container.querySelector('.test-validity');
    expect(validity.textContent).toBe('false');
  });

  it('should update its validity', () => {
    const callback = jest.fn();
    const { container } = render(
      <FormProvider onSubmit={callback}>
        <AutoForm.Control name="foo" />
        <DebugFormProvider />
      </FormProvider>
    );

    const input = container.querySelector('input');
    const validity = container.querySelector('.test-validity');
    expect(validity.textContent).toBe('false');
    act(() => { fireEvent.change(input, { target: { value: 'something' } }); });
    expect(validity.textContent).toBe('true');
  });

  it('should cast values to their specified types', () => {
    const callback = jest.fn(value => new Promise(resolve => resolve(value)));
    const { container } = render(
      <AutoForm onSubmit={callback}>
        <AutoForm.Control name="number" type="number" />
      </AutoForm>
    );

    const input = container.querySelector('input');
    act(() => { fireEvent.change(input, { target: { value: "42" }}); })
    const form = container.querySelector('form');
    act(() => { fireEvent.submit(form); });
    expect(callback).toBeCalledWith({ number: 42 });
  });

  it('should erase on error', async () => {
    const callback = _ => Promise.reject('lets reject this because reasons');
    const { container } = render(
      <AutoForm onSubmit={callback}>
        <AutoForm.Control
          name="failed"
          defaultValue="23"
          eraseOnFailure />
      </AutoForm>,
      container
    );

    const form = container.querySelector('form');
    fireEvent.submit(form);

    const input = container.querySelector('input');
    await wait(() => expect(input.value).toBe(''));

  });

  it('should present validity state only when edited', () => {
    const callback = jest.fn();
    const { container } = render(
      <FormProvider onSubmit={callback}>
        <AutoForm.Control name="foo" type="number" validator={x => x === 42}/>
        <DebugFormProvider />
      </FormProvider>
    );

    const input = container.querySelector('input');
    let validMarker = container.querySelector('input.is-invalid');
    expect(validMarker).toBeNull();
    fireEvent.change(input, { target: { value: 19 }});
    validMarker = container.querySelector('input.is-invalid');
    expect(validMarker).toBeDefined();
  });

});


describe('<AutoForm.Submit />', () => {

  it('should prevent manual submits when invalid', () => {
    const callback = jest.fn();

    const { container } = render(
      <AutoForm onSubmit={callback}>
        <AutoForm.Control name="foo" type="text" />
        <AutoForm.Submit>Submit</AutoForm.Submit>
      </AutoForm>
    );

    const button = container.querySelector('button');
    act(() => { fireEvent.click(button); });

    expect(callback).toHaveBeenCalledTimes(0);
  });

  it('should present an error feedback', async () => {
    const callback = _ => Promise.reject('nope');

    const { container } = render(
      <AutoForm onSubmit={callback}>
        <AutoForm.Control name="foo" type="text" optional/>
        <AutoForm.Submit>Submit</AutoForm.Submit>
      </AutoForm>
    );

    const form = container.querySelector('form');
    fireEvent.submit(form);

    const button= container.querySelector('button');
    await wait(() => expect(button.classList).toContain('btn-danger'));
  });

});


describe('<AutoForm.Switch />', () => {

  it('should block form submission', () => {
    const callback = jest.fn();

    const { container } = render(
      <AutoForm onSubmit={callback}>
        <AutoForm.Switch name="foo" label="activate me!" />
        <AutoForm.Submit>Submit</AutoForm.Submit>
      </AutoForm>
    );

    const button = container.querySelector('button');
    act(() => { fireEvent.click(button); });
    expect(callback).toHaveBeenCalledTimes(0);

    const input = container.querySelector('input');
    act(() => { fireEvent.click(input); });
    act(() => { fireEvent.click(button); });
    expect(callback).toHaveBeenCalled();
  });

  it('should not block form submission', () => {
    const callback = jest.fn();

    const { container } = render(
      <AutoForm onSubmit={callback}>
        <AutoForm.Switch name="foo" label="activate me!" optional />
        <AutoForm.Submit>Submit</AutoForm.Submit>
      </AutoForm>
    );

    const button = container.querySelector('button');
    act(() => { fireEvent.click(button); });
    expect(callback).toHaveBeenCalledTimes(1);

    const input = container.querySelector('input');
    act(() => { fireEvent.click(input); });
    act(() => { fireEvent.click(button); });
    expect(callback).toHaveBeenCalledTimes(2);
  });

});


describe('<AutoForm />', () => {

  it('should prevent manual submits when invalid', () => {
    const callback = jest.fn();

    const { container, getByText } = render(
      <AutoForm onSubmit={callback}>
        <AutoForm.Control name="foo" type="text" />
        <AutoForm.Submit>Submit</AutoForm.Submit>
      </AutoForm>
    );

    const button = container.querySelector('button');
    act(() => { fireEvent.click(button); });

    expect(callback).toHaveBeenCalledTimes(0);
  });

  it('should prevent programmed submits when invalid', () => {
    let error = null;
    const callback = value => {
      if (value instanceof Error)
        error = value;
      return new Promise(resolve => resolve());
    }

    const { container, getByText } = render(
      <AutoForm onSubmit={callback}>
        <AutoForm.Control name="foo" type="text" />
      </AutoForm>
    );

    const form = container.querySelector('form');
    act(() => { fireEvent.submit(form); });

    expect(error).toBeTruthy();
  });

  it('should handle erronous callbacks', async () => {
    const callback = jest.fn(() => { throw (new Error('why not')); });

    const { container, getByText } = render(
        <AutoForm onSubmit={callback}>
          <AutoForm.Control name="foo" type="text" optional />
          <AutoForm.Submit>Submit</AutoForm.Submit>
          <DebugFormProvider />
        </AutoForm>
    );

    const button = container.querySelector('button');
    act(() => { fireEvent.click(button); });

    const error = container.querySelector('.test-error');
    await wait(() => expect(error.textContent).toBe(Error('why not').toString()));
  });

  it('should accept default functions for submission', async () => {
    const callback = jest.fn(({ foo }) => foo + foo);

    const { container, getByText } = render(
        <AutoForm onSubmit={callback}>
          <AutoForm.Control name="foo" type="text" optional />
          <AutoForm.Submit>Submit</AutoForm.Submit>
          <DebugFormProvider />
        </AutoForm>
    );

    const form = container.querySelector('form');
    act(() => { fireEvent.submit(form); });

    const error = container.querySelector('.test-error');
    await wait(() => expect(error.textContent).toBe('false'));
  })

  it.skip('should fail when multiple inputs have the same name', () => {
    // TODO
  });

});
