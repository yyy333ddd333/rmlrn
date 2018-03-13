import { file, parse } from './globals.js'

function component() {
  var element = document.createElement('div');

  element.innerHTML = _.join(['Hello', 'webpack', file], ' ');
  // Assume we are in the context of `window`
  this.alert('Hmmm, this probably isn\'t a great idea...')

  return element;
}

document.body.appendChild(component());
