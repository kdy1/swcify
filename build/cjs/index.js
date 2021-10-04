'use strict';

Object.defineProperty(exports, '__esModule', { value: true });

var path = require('path');
var fs = require('fs');
var helper = require('@node-rs/helper');

// grabs the appropriate dist code for our platform
// ("swcify" is the name defined in package.json)
const bindings = helper.loadBinding(getBinaryDir(), 'swcify', 'swcify');
async function transform(src, options = {}) {
  const isModule = typeof src !== 'string';

  if (options && options.jsc && options.jsc.parser) {
    options.jsc.parser.syntax = options.jsc.parser.syntax || 'ecmascript';
  }

  return bindings.transform(isModule ? JSON.stringify(src) : src, isModule, toBuffer(options));
}
function transformSync(src, options = {}) {
  const isModule = typeof src !== 'string';

  if (options && options.jsc && options.jsc.parser) {
    options.jsc.parser.syntax = options.jsc.parser.syntax || 'ecmascript';
  }

  return bindings.transformSync(isModule ? JSON.stringify(src) : src, isModule, toBuffer(options));
}

function toBuffer(raw) {
  return Buffer.from(JSON.stringify(raw));
}

function getBinaryDir() {
  // 💩 we know that in built code we are nested an extra level from root.
  const pathToRoot = __dirname.endsWith('build/cjs') ? path.join(__dirname, '..', '..') : path.join(__dirname, '..'); // use the temp gitignored local builds if we have them otherwise use the canonical builds

  return fs.existsSync(path.join(pathToRoot, 'dev')) ? path.join(pathToRoot, 'dev') : path.join(pathToRoot, 'dist');
}

exports.transform = transform;
exports.transformSync = transformSync;
