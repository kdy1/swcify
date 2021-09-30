use ansi_term::Color;
use std::path::Path;
use std::{env, fs::read_to_string};
use swc_common::FileName;
use swc_ecma_transforms_testing::Tester;
use swc_ecmascript::ast::Module;
use swc_ecmascript::parser::{Parser, StringInput, Syntax};
use swc_ecmascript::transforms::{fixer, hygiene};
use swc_ecmascript::visit::FoldWith;
use testing::{assert_eq, DebugUsingDisplay, NormalizedOutput};

// Simplified version of swc_ecma_transforms_testing::test_fixture.
// Created because test_fixture() couldn't handle transforms that hold references to comments map.
// test_fixture also dropped comments
pub fn custom_test_fixture(
    syntax: Syntax,
    tranform: &(dyn Fn(Module, &mut Tester) -> Module),
    input: &Path,
    output: &Path,
) {
    let expected = read_to_string(output);
    let _is_really_expected = expected.is_ok();
    let expected = expected.unwrap_or_default();
    let expected_src = Tester::run(|tester| {
        let fm = &tester
            .cm
            .new_source_file(FileName::Real("output.js".into()), expected.into());

        let expected_module = {
            let mut p = Parser::new(syntax, StringInput::from(&**fm), Some(&tester.comments));
            let res = p
                .parse_module()
                .map_err(|e| e.into_diagnostic(&tester.handler).emit());

            for e in p.take_errors() {
                e.into_diagnostic(&tester.handler).emit()
            }

            res?
        };

        let expected_module = expected_module
            .fold_with(&mut hygiene())
            .fold_with(&mut fixer(Some(&tester.comments)));

        let expected_src = tester.print(&expected_module, &tester.comments.clone());

        println!(
            "----- {} -----\n{}",
            Color::Green.paint("Expected"),
            expected_src
        );

        Ok(expected_src)
    });

    let input_str = read_to_string(input);
    let _is_really_input = input_str.is_ok();
    let input_str = input_str.unwrap_or_default();
    let actual_src = Tester::run(|tester| {
        let fm = tester
            .cm
            .new_source_file(FileName::Real("input.js".into()), input_str.into());
        let input_str = read_to_string(input).unwrap();
        println!("----- {} -----\n{}", Color::Green.paint("Input"), input_str);

        println!("----- {} -----", Color::Green.paint("Actual"));

        let actual_module = {
            let mut p = Parser::new(syntax, StringInput::from(&*fm), Some(&tester.comments));
            let res = p
                .parse_module()
                .map_err(|e| e.into_diagnostic(&tester.handler).emit());

            for e in p.take_errors() {
                e.into_diagnostic(&tester.handler).emit()
            }
            res?
        };

        let actual_module = tranform(actual_module, tester);

        let actual_module = actual_module
            .fold_with(&mut hygiene())
            .fold_with(&mut fixer(Some(&tester.comments)));

        let actual_src = tester.print(&actual_module, &tester.comments.clone());

        Ok(actual_src)
    });
    let mut results = vec![];
    println!("{}", actual_src);
    if actual_src == expected_src {
        // Ignore `UPDATE`
        return;
    }
    if let Ok("1") = env::var("UPDATE").as_deref() {
        results.push(NormalizedOutput::from(actual_src.clone()).compare_to_file(output));
    }
    assert_eq!(
        DebugUsingDisplay(&actual_src),
        DebugUsingDisplay(&expected_src)
    );
    for result in results {
        result.unwrap();
    }
}
