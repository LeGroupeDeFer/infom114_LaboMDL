/**
 * @namespace validators
 * @memberof lib
 */


/**
 * Checks if the given email matches UNamur emails format.
 * @memberof lib.validators
 *
 * @param {string} email The email to verify
 * @returns boolean
 */
export function isUnamurEmail(email) {
  return /[a-zA-Z0-9._%+-]+@(student\.)?unamur\.be\s*$/.test(email);
}

/**
 * Check if the given password matches UNanimity passwords format.
 * @memberof lib.validators
 *
 * @param {string} password The password to verify
 * @returns boolean
 */
export function isValidPassword(password) {
  return password.length > 3;
}

/**
 * Check if the given number is a natural or can be casted as one.
 * @memberof lib.validators
 *
 * @param {string | number} number
 * @returns boolean
 */
export function isValidNatural(number) {
    const n = Number(number);
    return n >= 0 && n % 1 === 0;
}

export function isValidPhoneNumber(phoneNumber) {
  const fmt = /^[\+]?[(]?[0-9]{3}[)]?[-\s\.]?[0-9]{3}[-\s\.]?[0-9]{4,6}$/im;
  return fmt.test(phoneNumber);
}
