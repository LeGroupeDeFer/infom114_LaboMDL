import React, { Component } from 'react';
import ReactDOM from 'react-dom';
import '~/front/scss/style.scss';
import App from './App';

// Temporary code to justify Jest tests
const sum = (a, b) => a + b;
export default sum;

// Actual app
const root = document.getElementById('root');
ReactDOM.render(<App />, root);
