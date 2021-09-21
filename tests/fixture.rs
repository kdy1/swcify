use std::path::PathBuf;
use swc_ecma_transforms_testing::{test_fixture};
use swc_ecmascript::{
  parser::{EsConfig, Syntax},
};
use testing::fixture;

#[path = "../src/async_transform.rs"]
mod async_transform;

use async_transform::AsyncTransform;

fn syntax() -> Syntax {
  Syntax::Es(EsConfig {
    jsx: true,
    dynamic_import: true,
    ..Default::default()
  })
}

#[fixture("tests/fixture/async/**/input.js")]
fn async_fixture(input: PathBuf) {
  let output = input.parent().unwrap().join("output.js");
  test_fixture(syntax(), &|_tr| AsyncTransform::with_defaults(), &input, &output);
}
