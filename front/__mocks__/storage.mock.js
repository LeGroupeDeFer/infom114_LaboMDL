
let store = {};
const localStorage = {
  getItem: jest.fn(key => key in store ? store[key] : null),
  setItem: jest.fn((key, value) => { store[key] = value; }),
  removeItem: jest.fn(key => { store[key] = null; }),
  clear: jest.fn(() => { store = {}; })
};


Object.defineProperty(window, 'localStorage', {
  value: localStorage
});
