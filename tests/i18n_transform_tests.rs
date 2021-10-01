use std::path::PathBuf;
use swc_common::FileName;
use swc_ecmascript::parser::{EsConfig, Syntax};
use swc_ecmascript::visit::FoldWith;

mod util;

#[path = "../src/i18n_transform.rs"]
mod i18n_transform;

use i18n_transform::{i18n_transform, I18nMode};
use util::custom_test_fixture;

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        dynamic_import: true,
        ..Default::default()
    })
}

#[cfg(test)]
mod i18_tests {
    use super::*;
    #[test]
    fn injects_arguments_into_with_i18n_when_adjacent_exist() {
        i18n_fixture_test(
            "tests/fixtures/i18n/inputs/withI18n.js".into(),
            "tests/fixtures/i18n/outputs/withI18n_dynamic_paths.js".into(),
            "tests/fixtures/i18n/translations/adjacent/MyComponent.js".into(),
            I18nMode::WithDynamicPaths,
            "en".into(),
        );
    }
    #[test]
    fn injects_arguments_into_use_i18n_when_adjacent_exist() {
        i18n_fixture_test(
            "tests/fixtures/i18n/inputs/useI18n.js".into(),
            "tests/fixtures/i18n/outputs/useI18n_dynamic_paths.js".into(),
            "tests/fixtures/i18n/translations/adjacent/MyComponent.js".into(),
            I18nMode::WithDynamicPaths,
            "en".into(),
        );
    }
    #[test]
    fn does_not_inject_arguments_when_no_adjacent_exist() {
        i18n_fixture_test(
            "tests/fixtures/i18n/inputs/useI18n.js".into(),
            "tests/fixtures/i18n/inputs/useI18n.js".into(),
            "tests/fixtures/i18n/translations/NoTranslations.js".into(),
            I18nMode::WithDynamicPaths,
            "en".into(),
        );
    }
    #[test]
    fn does_not_transform_other_react_i18n_imports() {
        i18n_fixture_test(
            "tests/fixtures/i18n/inputs/other_i18n_import.js".into(),
            "tests/fixtures/i18n/inputs/other_i18n_import.js".into(),
            "tests/fixtures/i18n/translations/adjacent/MyComponent.js".into(),
            I18nMode::WithDynamicPaths,
            "en".into(),
        );
    }
    #[test]
    fn does_not_transform_use_i18n_imports_from_other_libraries() {
        i18n_fixture_test(
            "tests/fixtures/i18n/inputs/other_lib_useI18n_import.js".into(),
            "tests/fixtures/i18n/inputs/other_lib_useI18n_import.js".into(),
            "tests/fixtures/i18n/translations/adjacent/MyComponent.js".into(),
            I18nMode::WithDynamicPaths,
            "en".into(),
        );
    }
    #[test]
    fn does_not_transform_with_i18n_imports_from_other_libraries() {
        i18n_fixture_test(
            "tests/fixtures/i18n/inputs/other_lib_withI18n_import.js".into(),
            "tests/fixtures/i18n/inputs/other_lib_withI18n_import.js".into(),
            "tests/fixtures/i18n/translations/adjacent/MyComponent.js".into(),
            I18nMode::WithDynamicPaths,
            "en".into(),
        );
    }

    #[test]
    fn transforms_with_i18n_when_it_was_renamed_during_import() {
        i18n_fixture_test(
            "tests/fixtures/i18n/inputs/rename_withI18n.js".into(),
            "tests/fixtures/i18n/outputs/rename_withI18n_dynamic_paths.js".into(),
            "tests/fixtures/i18n/translations/adjacent/MyComponent.js".into(),
            I18nMode::WithDynamicPaths,
            "en".into(),
        );
    }
    #[test]
    fn transforms_use_i18n_when_it_was_renamed_during_import() {
        i18n_fixture_test(
            "tests/fixtures/i18n/inputs/rename_useI18n.js".into(),
            "tests/fixtures/i18n/outputs/rename_useI18n_dynamic_paths.js".into(),
            "tests/fixtures/i18n/translations/adjacent/MyComponent.js".into(),
            I18nMode::WithDynamicPaths,
            "en".into(),
        );
    }
    #[test]
    fn does_not_transform_with_i18n_when_it_already_has_arguments() {
        i18n_fixture_test(
            "tests/fixtures/i18n/inputs/with_args_withI18n.js".into(),
            "tests/fixtures/i18n/inputs/with_args_withI18n.js".into(),
            "tests/fixtures/i18n/translations/adjacent/MyComponent.js".into(),
            I18nMode::WithDynamicPaths,
            "en".into(),
        );
    }
    #[test]
    fn does_not_transform_use_i18n_when_it_already_has_arguments() {
        i18n_fixture_test(
            "tests/fixtures/i18n/inputs/with_args_useI18n.js".into(),
            "tests/fixtures/i18n/inputs/with_args_useI18n.js".into(),
            "tests/fixtures/i18n/translations/adjacent/MyComponent.js".into(),
            I18nMode::WithDynamicPaths,
            "en".into(),
        );
    }
    #[test]
    #[should_panic]
    fn throws_when_multiple_components_in_a_single_file_request_translations() {
        i18n_fixture_test(
            "tests/fixtures/i18n/inputs/multi_requests.js".into(),
            "tests/fixtures/i18n/inputs/multi_requests.js".into(),
            "tests/fixtures/i18n/translations/adjacent/MyComponent.js".into(),
            I18nMode::WithDynamicPaths,
            "en".into(),
        );
    }
    #[test]
    fn injects_arguments_with_translations_import_into_use_i18n_when_mode_equals_to_from_generated_index(
    ) {
        i18n_fixture_test(
            "tests/fixtures/i18n/inputs/useI18n.js".into(),
            "tests/fixtures/i18n/outputs/useI18n_generated_index.js".into(),
            "tests/fixtures/i18n/translations/adjacent/MyComponent.js".into(),
            I18nMode::FromGeneratedIndex,
            "en".into(),
        );
    }
    #[test]
    fn loads_fr_json_as_default_translation_when_default_locale_is_set_to_fr() {
        i18n_fixture_test(
            "tests/fixtures/i18n/inputs/withI18n.js".into(),
            "tests/fixtures/i18n/outputs/withI18n_dynamic_paths_fr.js".into(),
            "tests/fixtures/i18n/translations/adjacent/MyComponent.js".into(),
            I18nMode::WithDynamicPaths,
            "fr".into(),
        );
    }
    #[test]
    fn injects_a_dictionary_import_and_returns_dictionary_values_from_use_i18n() {
        i18n_fixture_test(
            "tests/fixtures/i18n/inputs/useI18n.js".into(),
            "tests/fixtures/i18n/outputs/useI18n_dictionary_index.js".into(),
            "tests/fixtures/i18n/translations/fallback_only/MyComponent.js".into(),
            I18nMode::FromDictionaryIndex,
            "en".into(),
        );
    }
    #[test]
    fn injects_proper_translation_function_for_fallback_plus_one_case() {
        i18n_fixture_test(
            "tests/fixtures/i18n/inputs/useI18n.js".into(),
            "tests/fixtures/i18n/outputs/useI18n_explict_paths_single.js".into(),
            "tests/fixtures/i18n/translations/fallback_plus_one/MyComponent.js".into(),
            I18nMode::WithExplicitPaths,
            "en".into(),
        );
    }
    #[test]
    fn injects_translation_function_with_multiple_static_imports() {
        i18n_fixture_test(
            "tests/fixtures/i18n/inputs/useI18n.js".into(),
            "tests/fixtures/i18n/outputs/useI18n_explict_paths_multiple.js".into(),
            "tests/fixtures/i18n/translations/adjacent/MyComponent.js".into(),
            I18nMode::WithExplicitPaths,
            "en".into(),
        );
    }
    fn i18n_fixture_test(
        input: PathBuf,
        output: PathBuf,
        filename: PathBuf,
        mode: I18nMode,
        default_locale: String,
    ) {
        custom_test_fixture(
            syntax(),
            &|module, tester| {
                let file = FileName::from(filename.clone());
                let module = module.fold_with(&mut i18n_transform(
                    file,
                    mode,
                    default_locale.clone(),
                    &tester.comments,
                ));
                module
            },
            &input,
            &output,
        );
    }
}
