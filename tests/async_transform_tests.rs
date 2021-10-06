use std::path::PathBuf;

use swc_ecma_transforms_testing::test_fixture;
use swc_ecmascript::parser::{EsConfig, Syntax};

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

#[cfg(test)]
mod async_transform_tests {
    use super::*;
    #[test]
    fn default_packages() {
        async_fixture_test("tests/fixtures/async/default_packages".into());
    }
    #[test]
    fn default_packages_multi() {
        async_fixture_test("tests/fixtures/async/default_packages_multi".into());
    }
    #[test]
    fn id_prop_exists() {
        async_fixture_test("tests/fixtures/async/id_prop_exists".into());
    }
    #[test]
    fn load_is_function_declaration() {
        async_fixture_test("tests/fixtures/async/load_is_function_declaration".into());
    }
    #[test]
    fn load_is_method() {
        async_fixture_test("tests/fixtures/async/load_is_method".into());
    }
    #[test]
    fn no_arguments() {
        async_fixture_test("tests/fixtures/async/no_arguments".into());
    }
    #[test]
    fn no_dynamic_import() {
        async_fixture_test("tests/fixtures/async/no_dynamic_import".into());
    }
    #[test]
    fn no_load_prop() {
        async_fixture_test("tests/fixtures/async/no_load_prop".into());
    }
    #[test]
    fn not_call_expression() {
        async_fixture_test("tests/fixtures/async/not_call_expression".into());
    }
    #[test]
    fn not_function() {
        async_fixture_test("tests/fixtures/async/not_function".into());
    }
    #[test]
    fn not_object() {
        async_fixture_test("tests/fixtures/async/not_object".into());
    }
    #[test]
    fn unrelated_function() {
        async_fixture_test("tests/fixtures/async/unrelated_function".into());
    }
    #[test]
    fn unrelated_function_call() {
        async_fixture_test("tests/fixtures/async/unrelated_function_call".into());
    }
    fn async_fixture_test(fixture_dir: PathBuf) {
        let input = fixture_dir.join("input.js");
        let output = fixture_dir.join("output.js");
        test_fixture(
            syntax(),
            &|_tr| AsyncTransform::with_defaults(),
            &input,
            &output,
        );
    }
}
