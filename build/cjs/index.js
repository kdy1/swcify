'use strict';

Object.defineProperty(exports, '__esModule', { value: true });

const path = require("path");

const {
  loadBinding
} = require("@node-rs/helper"); // grabs the appropriate native code for our platform
// ("swcify" is the name defined in package.json)


const nativeBindings = loadBinding(path.join(__dirname, "native"), "swcify", "swcify");
async function transform(src, options = {}) {
  var _options$jsc;

  const isModule = typeof src !== "string";

  if (options !== null && options !== void 0 && (_options$jsc = options.jsc) !== null && _options$jsc !== void 0 && _options$jsc.parser) {
    var _options$jsc$parser$s;

    options.jsc.parser.syntax = (_options$jsc$parser$s = options.jsc.parser.syntax) !== null && _options$jsc$parser$s !== void 0 ? _options$jsc$parser$s : "ecmascript";
  }

  return nativeBindings.transform(isModule ? JSON.stringify(src) : src, isModule, toBuffer(options));
}
function transformSync(src, options = {}) {
  var _options$jsc2;

  const isModule = typeof src !== "string";

  if (options !== null && options !== void 0 && (_options$jsc2 = options.jsc) !== null && _options$jsc2 !== void 0 && _options$jsc2.parser) {
    var _options$jsc$parser$s2;

    options.jsc.parser.syntax = (_options$jsc$parser$s2 = options.jsc.parser.syntax) !== null && _options$jsc$parser$s2 !== void 0 ? _options$jsc$parser$s2 : "ecmascript";
  }

  return nativeBindings.transformSync(isModule ? JSON.stringify(src) : src, isModule, toBuffer(options));
}

function toBuffer(raw) {
  return Buffer.from(JSON.stringify(raw));
}

exports.transform = transform;
exports.transformSync = transformSync;
