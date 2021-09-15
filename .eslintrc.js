/* eslint-env node */

module.exports = {
  extends: [
    "plugin:@shopify/typescript",
    "plugin:@shopify/react",
    "plugin:@shopify/jest",
    "plugin:@shopify/prettier",
  ],
  ignorePatterns: ["build/**/*"],
};
