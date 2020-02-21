import { trace, capitalize, debounce } from '../../js/utils';

it('`trace` returns the given value', () => {
  expect(trace(5)).toEqual(5);
  expect(trace([1, 2, 3])).toStrictEqual([1, 2, 3]);
  expect(trace({ key: 'value' })).toStrictEqual({ key: 'value' });
});

it('`capitalize` transforms the first letter', () => {
  expect(capitalize('w')).toBe('W');
  expect(capitalize('word')).toBe('Word');
  expect(capitalize('WORD')).toBe('Word');
  expect(capitalize('wORD')).toBe('Word');
  expect(capitalize(' wORD')).toBe(' Word');
  expect(capitalize('                  wORD')).toBe('                  Word');
  expect(capitalize('a sentence with words')).toBe('A sentence with words');
  expect(capitalize(' a sentence with words')).toBe(' A sentence with words');
});

it('`capitalize` does not transform empty strings', () => {
  expect(capitalize('')).toBe('');
  expect(capitalize(' ')).toBe(' ');
});

it('`capitalize` does not mutate the source string', () => {
  let xs = 'abc';
  capitalize(xs);
  expect(xs).toBe('abc');
});

it('`debounce` does not call more than once every second', () => {
  let x = 0;
  function increment() {
    x = x + 1;
  }
  const notSoFast = debounce(increment, 1000);
  const inFiveSeconds = new Date();
  inFiveSeconds.setSeconds(inFiveSeconds.getSeconds() + 10);

  while ((new Date()) < inFiveSeconds) {
    notSoFast();
  }

  expect(x).toBeLessThan(5);
});

it('`debounce` without timer does not change anything', () => {
  let x = 0;
  function increment() {
    x = x + 1;
  }
  const maybeFaster = debounce(increment, 0);
  for (let i = 0; i < 10; i++)
    maybeFaster();

  expect(x).toEqual(10);
});
