## SWCify

[![Build dist binaries](https://github.com/Shopify/swcify/actions/workflows/build_and_commit_binaries.yml/badge.svg)](https://github.com/Shopify/swcify/actions/workflows/build_and_commit_binaries.yml)
[![JS tests](https://github.com/Shopify/swcify/actions/workflows/js_test.yml/badge.svg)](https://github.com/Shopify/swcify/actions/workflows/js_test.yml)
[Shipit](https://shipit.shopify.io/shopify/swcify/production)

This repo wraps SWC with a napi-rs package implementing our own custom transforms.

### Toolchain

You'll need the rust nightly version listed in `rust-toolchain` in order to build this project.

### Using locally from a node project

This package is currently not deployed anywhere, so there is a fairly manual process to use it.

- clone the repo
- make your code changes
- `npx yalc publish`
- in your other repo `npx yalc install`
- `require('swcify')`;

> Note: If you want to run yalc without automatically building, use `npx yalc publish --no-scripts`

### Running JS tests

You can run tests against JS changes without building using `yarn test`, but if you want to E2E test rust changes make sure to use `yarn build:rust dev` first.
