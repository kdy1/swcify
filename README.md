## SWCify

[![Build native binaries](https://github.com/Shopify/swcify/actions/workflows/build_native.yml/badge.svg)](https://github.com/Shopify/swcify/actions/workflows/build_native.yml)
[![JS tests](https://github.com/Shopify/swcify/actions/workflows/js_test.yml/badge.svg)](https://github.com/Shopify/swcify/actions/workflows/js_test.yml)
[Shipit](https://shipit.shopify.io/shopify/swcify/production)

This repo wraps SWC with a napi-rs package implementing our own custom transforms.

### Toolchain

You'll need the rust nightly version listed in `rust-toolchain` in order to build this project.

### Using locally from a node project

This package is currently not deployed anywhere, so there is a fairly manual process to use it.

- clone the repo
- build the package `yarn build:debug`
- `npx yalc publish`
- in your other repo `npx yalc install`
- `require('swcify')`;

### Running tests

- `yarn build:test`
- `yarn test`
