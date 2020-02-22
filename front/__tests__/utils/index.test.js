import { trace, capitalize, preview, debounce, delay } from '../../js/utils';
import * as sinon from 'sinon';

describe('utils', () => {

  let clock;
  beforeEach(() => { clock = sinon.useFakeTimers(); });
  afterEach(() => { clock.restore(); });

  describe('trace', () => {
    it('should return the given value', () => {
      expect(trace(5)).toEqual(5);
      expect(trace([1, 2, 3])).toStrictEqual([1, 2, 3]);
      expect(trace({ key: 'value' })).toStrictEqual({ key: 'value' });
    });
  });

  describe('capitalize', () => {
    it('should transform the first letter', () => {
      expect(capitalize('w')).toBe('W');
      expect(capitalize('word')).toBe('Word');
      expect(capitalize('WORD')).toBe('Word');
      expect(capitalize('wORD')).toBe('Word');
      expect(capitalize(' wORD')).toBe(' Word');
      expect(capitalize('                  wORD')).toBe('                  Word');
      expect(capitalize('a sentence with words')).toBe('A sentence with words');
      expect(capitalize(' a sentence with words')).toBe(' A sentence with words');
    });

    it('should not transform empty strings', () => {
      expect(capitalize('')).toBe('');
      expect(capitalize(' ')).toBe(' ');
    });

    it('should not mutate the input', () => {
      let xs = 'abc';
      capitalize(xs);
      expect(xs).toBe('abc');
    });
  });

  describe('preview', () => {

    it('should trim to N characters', () => {
      expect(preview('Some string of 28 characters', 20)).toBe(
        'Some string of 28 ch...'
      );
    });

    it('should not change anything', () => {
      expect(preview('Some string of 28 characters', 28)).toBe(
        'Some string of 28 characters'
      );

      expect(preview('Some string of 28 characters', 50)).toBe(
        'Some string of 28 characters'
      );
    });

    it('should not do a thing on empty inputs', () => {
      expect(preview('', 10)).toEqual('');
    });

    it('should not mutate the input', () => {
      let xs = 'Some string of 28 characters';
      preview('Some string of 28 characters', 20);
      expect(xs).toBe('Some string of 28 characters');
    });

  });

  describe('debounce', () => {
    it('should supress consecutive calls', () => {
      const fn = jest.fn();
      const debounced = debounce(fn, 1000);

      debounced();
      expect(fn).toHaveBeenCalledTimes(0);

      for (let i = 0; i < 10; i++) {
        clock.tick(500);
        debounced();
      }

      expect(fn).toHaveBeenCalledTimes(0);

      clock.tick(1000);
      expect(fn).toHaveBeenCalledTimes(1);
    });

    it('should not change anything without timer', () => {
      const fn = jest.fn();
      const debounced = debounce(fn, 0);

      debounced();
      expect(fn).toHaveBeenCalledTimes(1);

      for (let i = 0; i < 10; i++) {
        clock.tick(500);
        debounced();
      }

      expect(fn).toHaveBeenCalledTimes(11);
      clock.tick(1000); // already called, shouldn't change anything
      expect(fn).toHaveBeenCalledTimes(11);
    });
  });

  describe('delay', () => {
    it('should delay the function call by 1 second', () => {
      const fn = jest.fn();
      const delayed = delay(fn, 1000);

      delayed();
      expect(fn).toHaveBeenCalledTimes(0);

      clock.tick(1000);
      expect(fn).toHaveBeenCalledTimes(1);
    });

    it('should not change anything without timer', () => {
      const fn = jest.fn();
      const delayed = delay(fn, 0);

      delayed();
      expect(fn).toHaveBeenCalledTimes(1);

      clock.tick(1000);
      expect(fn).toHaveBeenCalledTimes(1);
    });
  });

});
