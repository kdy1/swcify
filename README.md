## SWCify

[![Build dist binaries](https://github.com/Shopify/swcify/actions/workflows/build_and_commit_binaries.yml/badge.svg)](https://github.com/Shopify/swcify/actions/workflows/build_and_commit_binaries.yml)
[![JS tests](https://github.com/Shopify/swcify/actions/workflows/js_test.yml/badge.svg)](https://github.com/Shopify/swcify/actions/workflows/js_test.yml)

This repo wraps SWC with a napi-rs package implements custom transforms used by Shopify.

### Build Toolchain

1. [Install Yarn](https://classic.yarnpkg.com/lang/en/docs/install) and run:

   `yarn install`

2. Setup Rust

   - [Install Rustup](https://www.rust-lang.org/tools/install)
   - You'll need the rust nightly version listed in `rust-toolchain` in order to build this project. To install:

     `rustup toolchain install nightly`

     or set as default with:

     `rustup default nightly`

3. **Mac:** You may need to intall dev tools/dependencies with:

   `xcode-select --install`

4. Build with:

   `yarn build`

### Using locally from a node project

This package is currently not deployed anywhere, so there is a fairly manual process to use it.

- clone the repo
- Setup the build toolchain (see above)
- make your code changes
- `npx yalc publish`
- in your other repo `npx yalc install`
- `require('swcify')`;

> Note: If you want to run yalc without automatically building, use `npx yalc publish --no-scripts`

### Running Rust tests

To run rust tests use: `cargo test` or `yarn test:native`

### Running JS tests

You can run tests against JS changes without building using `yarn test:js`, but if you want to E2E test rust changes make sure to use `yarn build:rust dev` first.
