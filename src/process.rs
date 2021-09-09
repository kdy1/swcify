use anyhow::{bail, Error};
use std::sync::Arc;
use swc::config::{BuiltConfig, Options};
use swc::{Compiler, TransformOutput};
use swc_common::{comments::Comment, BytePos, SourceFile};
use swc_ecmascript::transforms::helpers::{self, Helpers};
use swc_ecmascript::utils::HANDLER;
use swc_ecmascript::visit::FoldWith;

// Processes JS with added plugins (someday)
pub fn process_js_shopify(
  compiler: &Arc<Compiler>,
  source: Arc<SourceFile>,
  options: &Options,
) -> Result<TransformOutput, Error> {
  let config = compiler.run(|| compiler.config_for_file(options, &source.name))?;
  let config = match config {
    Some(v) => v,
    None => {
      bail!("cannot process file because it's ignored by .swcrc")
    }
  };
  let config = BuiltConfig {
    // pass: chain!(hook_optimizer(), config.pass), // run custom transforms around the main swc pass
    pass: config.pass,
    syntax: config.syntax,
    target: config.target,
    minify: config.minify,
    external_helpers: config.external_helpers,
    source_maps: config.source_maps,
    input_source_map: config.input_source_map,
    is_module: config.is_module,
    output_path: config.output_path,
  };

  let program = compiler.parse_js(
    source.clone(),
    config.target,
    config.syntax,
    config.is_module,
    true,
  )?;

  compiler.run(|| {
    if config.minify {
      let preserve_excl = |_: &BytePos, vc: &mut Vec<Comment>| -> bool {
        vc.retain(|c: &Comment| c.text.starts_with("!"));
        !vc.is_empty()
      };
      compiler.comments().leading.retain(preserve_excl);
      compiler.comments().trailing.retain(preserve_excl);
    }
    let mut pass = config.pass;
    let program = helpers::HELPERS.set(&Helpers::new(config.external_helpers), || {
      HANDLER.set(&compiler.handler, || {
        // Fold module
        program.fold_with(&mut pass)
      })
    });

    compiler.print(
      &program,
      config.output_path,
      config.target,
      config.source_maps,
      None, // TODO: figure out sourcemaps
      config.minify,
    )
  })
}
