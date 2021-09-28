// This file is mostly a straight copy of the same in
// https://github.com/swc-project/swc/blob/master/node/binding/src
// as such we retain their license in LICENSE.md in this folder

use crate::async_transform::AsyncTransform;
use crate::{
    complete_output, get_compiler,
    util::{CtxtExt, MapErr},
};
use anyhow::{Context as _, Error};
use napi::{CallContext, Env, JsBoolean, JsObject, JsString, Task};
use std::sync::Arc;
use swc::config::Options;
use swc::{try_with_handler, Compiler, TransformOutput};
use swc_common::{FileName, SourceFile};
use swc_ecmascript::ast::Program;
use swc_ecmascript::transforms::pass::noop;
use swc_ecmascript::visit::Fold;

/// Input to transform
#[derive(Debug)]
pub enum Input {
    /// Raw source code.
    Source(Arc<SourceFile>),
}

pub struct TransformTask {
    pub c: Arc<Compiler>,
    pub input: Input,
    pub options: Options,
}

impl Task for TransformTask {
    type Output = TransformOutput;
    type JsValue = JsObject;

    fn compute(&mut self) -> napi::Result<Self::Output> {
        try_with_handler(self.c.cm.clone(), |handler| {
            self.c.run(|| match self.input {
                Input::Source(ref s) => {
                    let (before_pass, after_pass) = custom_transforms();
                    self.c.process_js_with_custom_pass(
                        s.clone(),
                        &handler,
                        &self.options,
                        before_pass,
                        after_pass,
                    )
                }
            })
        })
        .convert_err()
    }

    fn resolve(self, env: Env, result: Self::Output) -> napi::Result<Self::JsValue> {
        complete_output(&env, result)
    }
}

/// returns `compiler, (src / path), options, plugin, callback`
pub fn schedule_transform<F>(cx: CallContext, op: F) -> napi::Result<JsObject>
where
    F: FnOnce(&Arc<Compiler>, String, bool, Options) -> TransformTask,
{
    let c = get_compiler(&cx);

    let s = cx.get::<JsString>(0)?.into_utf8()?.as_str()?.to_owned();
    let is_module = cx.get::<JsBoolean>(1)?;
    let options: Options = cx.get_deserialized(2)?;

    let task = op(&c, s, is_module.get_value()?, options);

    cx.env.spawn(task).map(|t| t.promise_object())
}

pub fn exec_transform<F>(cx: CallContext, op: F) -> napi::Result<JsObject>
where
    F: FnOnce(&Compiler, String, &Options) -> Result<Arc<SourceFile>, Error>,
{
    let c = get_compiler(&cx);

    let s = cx.get::<JsString>(0)?.into_utf8()?;
    let is_module = cx.get::<JsBoolean>(1)?;
    let options: Options = cx.get_deserialized(2)?;

    let output = try_with_handler(c.cm.clone(), |handler| {
        c.run(|| {
            if is_module.get_value()? {
                let program: Program =
                    serde_json::from_str(s.as_str()?).context("failed to deserialize Program")?;
                c.process_js(&handler, program, &options)
            } else {
                let fm =
                    op(&c, s.as_str()?.to_string(), &options).context("failed to load file.")?;
                let (before_pass, after_pass) = custom_transforms();
                c.process_js_with_custom_pass(fm, &handler, &options, before_pass, after_pass)
            }
        })
    })
    .convert_err()?;

    complete_output(cx.env, output)
}

#[js_function(4)]
pub fn transform(cx: CallContext) -> napi::Result<JsObject> {
    schedule_transform(cx, |c, src, _, options| {
        let input = Input::Source(c.cm.new_source_file(
            if options.filename.is_empty() {
                FileName::Anon
            } else {
                FileName::Real(options.filename.clone().into())
            },
            src,
        ));
        TransformTask {
            c: c.clone(),
            input,
            options,
        }
    })
}

#[js_function(4)]
pub fn transform_sync(cx: CallContext) -> napi::Result<JsObject> {
    exec_transform(cx, |c, src, options| {
        Ok(c.cm.new_source_file(
            if options.filename.is_empty() {
                FileName::Anon
            } else {
                FileName::Real(options.filename.clone().into())
            },
            src,
        ))
    })
}

fn custom_transforms() -> (impl Fold, impl Fold) {
    let before_pass = AsyncTransform::with_defaults();
    let after_pass = noop();
    (before_pass, after_pass)
}
