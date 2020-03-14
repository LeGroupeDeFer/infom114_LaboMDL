import { isValidNatural, isValidPassword, isUnamurEmail } from '../../js/lib/validators'

describe('isValidPassword', () => {
  it('should succeed', () => {
    expect(isValidPassword('secret')).toBeTruthy();
    expect(isValidPassword('secret42')).toBeTruthy();
    expect(isValidPassword('abcdefghhfed')).toBeTruthy();
    expect(isValidPassword('abcdefghhfedghjfr654ezf/*é"914ez65f14cq36é&*aDZAQSDVG')).toBeTruthy();
  });

  it('should fail', () => {
    expect(isValidPassword('1')).toBeFalsy();
    expect(isValidPassword('123')).toBeFalsy();
    expect(isValidPassword('sec')).toBeFalsy();
  });
});

describe('isValidUnamurEmail', () => {
  it('should succeed', () => {
    expect(isUnamurEmail('john@unamur.be')).toBeTruthy();
    expect(isUnamurEmail('john.doe@unamur.be')).toBeTruthy();
    expect(isUnamurEmail('jdoe@student.unamur.be')).toBeTruthy();
    expect(isUnamurEmail('john.doe.the.famous.random.guy@student.unamur.be')).toBeTruthy();
    expect(isUnamurEmail('kevin.du.47@student.unamur.be')).toBeTruthy();
    expect(isUnamurEmail('a@unamur.be')).toBeTruthy();
  });

  it('should fail', () => {
    expect(isUnamurEmail('')).toBeFalsy();
    expect(isUnamurEmail('@unamur.be')).toBeFalsy();
    expect(isUnamurEmail('@student.unamur.be')).toBeFalsy();
    expect(isUnamurEmail('@unamur')).toBeFalsy();
    expect(isUnamurEmail('john@student.unamur')).toBeFalsy();
    expect(isUnamurEmail('jdoe@student.unamur.')).toBeFalsy();
    expect(isUnamurEmail('@student.unamur.')).toBeFalsy();
    expect(isUnamurEmail('@something.unamur.be')).toBeFalsy();
    expect(isUnamurEmail('jdoe@something.unamur.be')).toBeFalsy();
    expect(isUnamurEmail('jdoe@unamur.b')).toBeFalsy();
    expect(isUnamurEmail('jdoe@unamur.com')).toBeFalsy();
    expect(isUnamurEmail('jdoe@student.unamur.com')).toBeFalsy();
  });
});

describe('isValidNatural', () => {
  // It's as simple of a test as it gets, too lazy to find a better name
  it('should succeed', () => {
    expect(isValidNatural(0)).toBeTruthy();
    expect(isValidNatural(42)).toBeTruthy();
    expect(isValidNatural(10e3)).toBeTruthy();
    expect(isValidNatural(10e6)).toBeTruthy();
    expect(isValidNatural('12')).toBeTruthy();
  });

  it('should fail', () => {
    expect(isValidNatural(-1)).toBeFalsy();
    expect(isValidNatural(0.12)).toBeFalsy();
    expect(isValidNatural(-0.12)).toBeFalsy();
    expect(isValidNatural('whatever')).toBeFalsy();
    expect(isValidNatural(Date())).toBeFalsy();
    expect(isValidNatural(/iHaveNothingToDoHere/)).toBeFalsy();
    expect(isValidNatural(function(){})).toBeFalsy();
  });
});