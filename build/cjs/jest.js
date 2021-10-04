'use strict';

Object.defineProperty(exports, '__esModule', { value: true });

var process = require('process');
var fs = require('fs');
var path = require('path');
var index = require('./index.js');

let memoizedOptions;
const transformer = {
  process(src, filename, jestConfig) {
    if (memoizedOptions == null) {
      const options = jestConfig.transformerConfig || swcRcConfig();
      set(options, 'jsc.transform.hidden.jest', true);
      memoizedOptions = options;
    }

    return index.transformSync(src, { ...memoizedOptions,
      filename
    });
  }

};

function swcRcConfig() {
  const swcrc = path.join(process.cwd(), '.swcrc');
  const options = fs.existsSync(swcrc) ? JSON.parse(fs.readFileSync(swcrc, 'utf-8')) : {};
  return options;
}

function set(obj, path, value) {
  const parents = path.split('.');
  const key = parents.pop();
  let currentTarget = obj;

  for (const prop of parents) {
    if (currentTarget[prop] == null) {
      currentTarget[prop] = {};
    }

    currentTarget = currentTarget[prop];
  }

  currentTarget[key] = value;
}

exports['default'] = transformer;
