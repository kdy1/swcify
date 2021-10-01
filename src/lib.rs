// This file is mostly a straight copy of the same in
// https://github.com/swc-project/swc/blob/master/node/binding/src
// as such we retain their license in LICENSE.md in this folder

#![recursion_limit = "2048"]

#[macro_use]
extern crate napi_derive;
/// Explicit extern crate to use allocator.
extern crate swc_node_base;
extern crate maplit;

use backtrace::Backtrace;
use napi::{CallContext, Env, JsObject, JsUndefined};
use std::{env, panic::set_hook, sync::Arc};
use swc::{Compiler, TransformOutput};
use swc_common::{
  self,
  sync::Lazy,
  FilePathMapping, SourceMap,
};

mod transform;
mod util;
mod async_transform;
mod i18n_transform;

static COMPILER: Lazy<Arc<Compiler>> = Lazy::new(|| {
  let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));

  Arc::new(Compiler::new(cm.clone()))
});

#[module_exports]
fn init(mut exports: JsObject) -> napi::Result<()> {
  // Provide rust stack traces when SWC_DEBUG is present in env
  if cfg!(debug_assertions) || env::var("SWC_DEBUG").unwrap_or_default() == "1" {
    set_hook(Box::new(|panic_info| {
      let backtrace = Backtrace::new();
      println!("Panic: {:?}\nBacktrace: {:?}", panic_info, backtrace);
    }));
  }

  // Generate our node bindings
  exports.create_named_method("transform", transform::transform)?;
  exports.create_named_method("transformSync", transform::transform_sync)?;

  Ok(())
}

fn get_compiler(_ctx: &CallContext) -> Arc<Compiler> {
  COMPILER.clone()
}

#[js_function]
fn construct_compiler(ctx: CallContext) -> napi::Result<JsUndefined> {
  // TODO: Assign swc::Compiler
  ctx.env.get_undefined()
}

pub fn complete_output(env: &Env, output: TransformOutput) -> napi::Result<JsObject> {
  env.to_js_value(&output)?.coerce_to_object()
}

pub type ArcCompiler = Arc<Compiler>;
