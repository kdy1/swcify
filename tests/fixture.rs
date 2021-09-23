use rstest::rstest;
use std::path::PathBuf;
use swc_common::FileName;
use swc_ecma_transforms_testing::test_fixture;
use swc_ecmascript::parser::{EsConfig, Syntax};

#[path = "../src/async_transform.rs"]
mod async_transform;

#[path = "../src/i18n_transform.rs"]
mod i18n_transform;

use async_transform::AsyncTransform;
use i18n_transform::{I18nMode, I18nTransform};

fn syntax() -> Syntax {
  Syntax::Es(EsConfig {
    jsx: true,
    dynamic_import: true,
    ..Default::default()
  })
}

#[rstest]
#[case::default_packages("tests/fixture/async/default_packages")]
#[case::default_packages_multi("tests/fixture/async/default_packages_multi")]
#[case::id_prop_exists("tests/fixture/async/id_prop_exists")]
#[case::load_is_function_declaration("tests/fixture/async/load_is_function_declaration")]
#[case::load_is_method("tests/fixture/async/load_is_method")]
#[case::no_arguments("tests/fixture/async/no_arguments")]
#[case::no_dynamic_import("tests/fixture/async/no_dynamic_import")]
#[case::no_load_prop("tests/fixture/async/no_load_prop")]
#[case::not_call_expression("tests/fixture/async/not_call_expression")]
#[case::not_function("tests/fixture/async/not_function")]
#[case::not_object("tests/fixture/async/not_object")]
#[case::unrelated_function("tests/fixture/async/unrelated_function")]
#[case::unrelated_function_call("tests/fixture/async/unrelated_function_call")]
fn async_fixture_test(#[case] fixture_dir: PathBuf) {
  let input = fixture_dir.join("input.js");
  let output = fixture_dir.join("output.js");
  test_fixture(
    syntax(),
    &|_tr| AsyncTransform::with_defaults(),
    &input,
    &output,
  );
}

#[rstest]
#[case::injects_arguments_into_with_i18n_when_adjacent_exist(
  "tests/fixture/i18n/inputs/withI18n.js",
  "tests/fixture/i18n/outputs/withI18n_dynamic_paths.js",
  "tests/fixture/i18n/translations/adjacent/MyComponent.js",
  I18nMode::WithDynamicPaths,
  "en"
)]
#[case::injects_arguments_into_use_i18n_when_adjacent_exist(
  "tests/fixture/i18n/inputs/useI18n.js",
  "tests/fixture/i18n/outputs/useI18n_dynamic_paths.js",
  "tests/fixture/i18n/translations/adjacent/MyComponent.js",
  I18nMode::WithDynamicPaths,
  "en"
)]
#[case::does_not_inject_arguments_when_no_adjacent_exist(
  "tests/fixture/i18n/inputs/useI18n.js",
  "tests/fixture/i18n/inputs/useI18n.js",
  "tests/fixture/i18n/translations/MyComponent.js",
  I18nMode::WithDynamicPaths,
  "en"
)]
#[case::does_not_transform_other_react_i18n_imports(
  "tests/fixture/i18n/inputs/other_i18n_import.js",
  "tests/fixture/i18n/inputs/other_i18n_import.js",
  "tests/fixture/i18n/translations/adjacent/MyComponent.js",
  I18nMode::WithDynamicPaths,
  "en"
)]
#[case::does_not_transform_withI18n_imports_from_other_libraries(
  "tests/fixture/i18n/inputs/other_lib_useI18n_import.js",
  "tests/fixture/i18n/inputs/other_lib_useI18n_import.js",
  "tests/fixture/i18n/translations/adjacent/MyComponent.js",
  I18nMode::WithDynamicPaths,
  "en"
)]
#[case::does_not_transform_withI18n_imports_from_other_libraries(
  "tests/fixture/i18n/inputs/other_lib_withI18n_import.js",
  "tests/fixture/i18n/inputs/other_lib_withI18n_import.js",
  "tests/fixture/i18n/translations/adjacent/MyComponent.js",
  I18nMode::WithDynamicPaths,
  "en"
)]
// TODO: transforms_withI18n_when_it_was_renamed_during_import
// TODO: transforms_useI18n_when_it_was_renamed_during_import
// TODO: does_not_transform_withI18n_when_it_already_has_arguments
// TODO: does_not_transform_useI18n_when_it_already_has_arguments
// TODO: throws_when_multiple_components_in_a_single_file_request_translations
#[case::injects_arguments_with_translations_import_into_use_i18n_when_mode_equals_to_from_generated_index(
  "tests/fixture/i18n/inputs/useI18n.js",
  "tests/fixture/i18n/outputs/useI18n_generated_index.js",
  "tests/fixture/i18n/translations/adjacent/MyComponent.js",
  I18nMode::FromGeneratedIndex,
  "en"
)]
#[case::loads_fr_json_as_default_translation_when_default_locale_is_set_to_fr(
  "tests/fixture/i18n/inputs/withI18n.js",
  "tests/fixture/i18n/outputs/withI18n_dynamic_paths_fr.js",
  "tests/fixture/i18n/translations/adjacent/MyComponent.js",
  I18nMode::WithDynamicPaths,
  "fr"
)]
#[case::injects_a_dictionary_import_and_returns_dictionary_values_from_use_i18n(
  "tests/fixture/i18n/inputs/useI18n.js",
  "tests/fixture/i18n/outputs/useI18n_explict_paths_fallback.js",
  "tests/fixture/i18n/translations/fallback_only/MyComponent.js",
  I18nMode::WithExplicitPaths,
  "en"
)]
#[case::injects_proper_translation_function_for_fallback_plus_one_case(
  "tests/fixture/i18n/inputs/useI18n.js",
  "tests/fixture/i18n/outputs/useI18n_explict_paths_single.js",
  "tests/fixture/i18n/translations/fallback_plus_one/MyComponent.js",
  I18nMode::WithExplicitPaths,
  "en"
)]
#[case::injects_translation_function_with_multiple_static_imports(
  "tests/fixture/i18n/inputs/useI18n.js",
  "tests/fixture/i18n/outputs/useI18n_explict_paths_multiple.js",
  "tests/fixture/i18n/translations/adjacent/MyComponent.js",
  I18nMode::WithExplicitPaths,
  "en"
)]
fn i18n_fixture_test(
  #[case] input: PathBuf,
  #[case] output: PathBuf,
  #[case] filename: PathBuf,
  #[case] mode: I18nMode,
  #[case] default_locale: String,
) {
  test_fixture(
    syntax(),
    &|_tr| {
      let file = FileName::from(filename.clone());
      I18nTransform::new(file, mode, default_locale.clone())
    },
    &input,
    &output,
  );
}
