use std::path::PathBuf;
use swc_common::FileName;
use swc_ecmascript::parser::{EsConfig, Syntax};
use swc_ecmascript::visit::FoldWith;

mod util;

#[path = "../src/webworker_transform.rs"]
mod webworker_transform;

use webworker_transform::WebWorkerTransform;

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        dynamic_import: true,
        ..Default::default()
    })
}

#[cfg(test)]
mod webworker_tests {
    use super::*;
    #[test]
    fn placeholder() {}
}
