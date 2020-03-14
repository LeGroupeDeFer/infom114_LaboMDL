import 'regenerator-runtime';
import { act, fireEvent, render } from '@testing-library/react';
import React from 'react';
import ReactDOM from 'react-dom';
import AutoForm from '../../js/components/AutoForm';
import { FormProvider, useForm } from '../../js/components/AutoForm/formContext';
import SynchronousPromise from 'synchronous-promise';

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
        <AutoForm.Control name="number" type="number" defaultValue="42" />
      </AutoForm>
    );

    const form = container.querySelector('form');
    act(() => { fireEvent.submit(form); });
    expect(callback).toBeCalledWith({ number: 42 });
  });

  it.skip('should erase on error', async () => {
    // FIXME - The function actually work but getting jest to wait for the
    // submission promise to resolve is complicated, the callback is executed
    // from within react so we can't "await" it for completion.
    const container = document.createElement('div');
    document.body.appendChild(container);

    const callback = value => Promise.reject('lets reject this because reasons');

    await act(async () => {
      ReactDOM.render(
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
    });

    const input = container.querySelector('input');
    expect(input.value).toBe('');
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

  it.skip('should present an error feedback', () => {
    // FIXME - Same as before, the function works but I can't get jest to wait
    const callback = _ => Promise.reject('nope');

    const { container } = render(
      <AutoForm onSubmit={callback}>
        <AutoForm.Control name="foo" type="text" />
        <AutoForm.Submit>Submit</AutoForm.Submit>
      </AutoForm>
    );

    const button = container.querySelector('button');
    act(() => { fireEvent.click(button); });

    expect(button.classList).toContain('btn-danger');
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

  it('should handle erronous callbacks', () => {
    const callback = () => { throw (new Error('why not')); };

    const { container } = render(
      <AutoForm onSubmit={callback}>
        <AutoForm.Control name="foo" type="text" optional />
        <AutoForm.Submit>Submit</AutoForm.Submit>
      </AutoForm>
    );

    const button = container.querySelector('button');
    act(() => { fireEvent.click(button); });

    // FIXME - Assert error boundary
    expect(callback).toHaveBeenCalledTimes(0);
  });

  it.skip('should fail when multiple inputs have the same name', () => {
    // TODO
  });

})