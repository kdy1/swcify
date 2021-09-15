/* eslint-env node */

global.__SWCIFY_ROOT_DIR = __dirname;

function interopRequireDefault(obj) {
  return obj && obj.__esModule ? obj : { default: obj };
}
module.exports = interopRequireDefault(require("./build/cjs/index"));
