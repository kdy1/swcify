use serde::Deserialize;
use std::fs::read_to_string;
use std::path::PathBuf;
use swc_ecma_transforms_testing::test_fixture;
use swc_ecmascript::parser::{EsConfig, Syntax};

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        dynamic_import: true,
        ..Default::default()
    })
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TestConfig {
    #[serde(default)]
    throws: Option<String>,
    #[serde(default)]
    use_entry: bool,
}

#[testing::fixture("tests/fixture/web-worker/**/input.js")]
fn fixture(input: PathBuf) {
    let output = input.parent().unwrap().join("output.js");
    let config = read_to_string(input.parent().unwrap().join("config.json")).unwrap_or_else(|_| {
        // File not found
        "{}".to_string()
    });
    // TODO(kdy1): Use config.
    let _config: TestConfig = serde_json::from_str(&config).expect("failed to parse config.json");

    test_fixture(
        syntax(),
        &|_tr| swcify::web_worker::WebWorker::default(),
        &input,
        &output,
    );
}
