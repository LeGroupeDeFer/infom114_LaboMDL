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
  return /@(student\.)?unamur\.be\s*$/.test(email);
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
  try {
    const n = Number(number);
    return n >= 0 && n % 1 === 0;
  } catch {
    return false;
  }
}
