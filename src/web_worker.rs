use std::fmt::Debug;
use std::mem::take;

use rustc_hash::FxHashMap;
use serde::Serialize;
use swc_atoms::js_word;
use swc_common::comments::CommentKind;
use swc_common::comments::Comments;
use swc_common::Spanned;
use swc_common::DUMMY_SP;
use swc_ecmascript::visit::noop_fold_type;
use swc_ecmascript::visit::noop_visit_type;
use swc_ecmascript::{
    ast::*,
    minifier::{
        eval::{EvalResult, Evaluator},
        marks::Marks,
    },
    utils::{ident::IdentLike, prepend_stmts, private_ident, ExprExt, Id},
    visit::{Fold, FoldWith, Node, Visit, VisitWith},
};

#[derive(Default)]
pub struct Config {
    pub noop: bool,
}

pub fn web_worker<C>(config: Config, comments: C) -> WebWorker<C>
where
    C: Comments + Debug,
{
    WebWorker {
        config,
        comments,
        data: Default::default(),
        eval: Default::default(),
        added_imports: Default::default(),
    }
}

pub struct WebWorker<C>
where
    C: Comments + Debug,
{
    config: Config,
    comments: C,
    data: Data,
    eval: Option<Evaluator>,
    added_imports: Vec<ModuleItem>,
}

struct ImportAnalyzer<'a> {
    data: &'a mut Data,
}

#[derive(Default)]
struct Data {
    /// All import specifiers of `createWorkerFactory`.
    ///
    ///  - Value means plain
    create_worker_factory: FxHashMap<Id, bool>,
}

/// Options of **webpack** loaders.
#[derive(Debug, Serialize)]
struct LoaderOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    plain: bool,
}

impl<C> WebWorker<C>
where
    C: Comments + Debug,
{
    /// Returns `Some(plain)` if calleed is `createWorkerFactory`.
    fn extract_call_to_create_worker_factory(&mut self, callee: &Expr) -> Option<bool> {
        match callee {
            Expr::Ident(callee) => self
                .data
                .create_worker_factory
                .get(&callee.to_id())
                .copied(),

            _ => None,
        }
    }

    fn extract_import_from_arrow_arg(&mut self, args: &[ExprOrSpread]) -> Option<Str> {
        if args.len() == 1 && args[0].spread.is_none() {
            match &*args[0].expr {
                Expr::Arrow(ArrowExpr {
                    params,
                    body: BlockStmtOrExpr::Expr(body),
                    is_async: false,
                    is_generator: false,
                    ..
                }) if params.is_empty() => {
                    // We are in some arrow like `() => expr`.
                    //
                    // Now we check if the `expr` is a call to import.

                    match &**body {
                        Expr::Call(CallExpr {
                            callee: ExprOrSuper::Expr(callee),
                            args,
                            ..
                        }) => {
                            if callee.is_ident_ref_to(js_word!("import")) {
                                // We are in some arrow like `() => import(<args>)`.

                                if args.len() == 1 && args[0].spread.is_none() {
                                    let eval_res = self.eval.as_mut().unwrap().eval(&args[0].expr);

                                    match eval_res {
                                        Some(EvalResult::Lit(Lit::Str(s))) => return Some(s),
                                        res => {
                                            panic!("Failed to evaluate a dynamic import to a string to create a web worker: eval result = {:?}",res)
                                        }
                                    }
                                } else {
                                    // TODO(kdy1): Should we report an error?
                                }
                            }
                        }

                        _ => {}
                    }
                }

                _ => {}
            }
        }

        None
    }
}

impl<C> Fold for WebWorker<C>
where
    C: Comments + Debug,
{
    noop_fold_type!();

    fn fold_call_expr(&mut self, e: CallExpr) -> CallExpr {
        let mut e = e.fold_children_with(self);

        match &e.callee {
            ExprOrSuper::Expr(callee) => {
                if let Some(plain) = self.extract_call_to_create_worker_factory(&callee) {
                    let callee_span = callee.span();

                    if let Some(s) = self.extract_import_from_arrow_arg(&e.args) {
                        // import workerStuff from '@shopify/web-worker/webpack-loader!./worker';
                        // createWorkerFactory(workerStuff);

                        let mut loader_opts = LoaderOptions { plain, name: None };
                        // Parse `webpackChunkName` in comments.
                        if let Some(comments) = self.comments.get_leading(s.span.lo) {
                            for c in comments {
                                if matches!(c.kind, CommentKind::Line) {
                                    continue;
                                }

                                let s = c.text.trim();
                                if let Some(s) = s.strip_prefix("webpackChunkName:") {
                                    // Check for "foo"

                                    let stripped = s
                                        .trim()
                                        .strip_prefix('"')
                                        .and_then(|s| s.strip_suffix('"'))
                                        .or_else(|| {
                                            // Allow using 'foo'
                                            s.trim()
                                                .strip_suffix('\'')
                                                .and_then(|s| s.strip_prefix('\''))
                                        });

                                    if let Some(s) = stripped {
                                        if loader_opts.name.is_some() {
                                            panic!("`webpackChunkName:` can be specified only once")
                                        }
                                        loader_opts.name = Some(s.into());
                                    }
                                }
                            }
                        }

                        let options_json = serde_json::to_string(&loader_opts).unwrap();

                        let src = Str {
                            span: s.span,
                            value: format!(
                                "@shopify/web-worker/webpack-loader?{}!{}",
                                options_json, s.value
                            )
                            .into(),
                            has_escape: false,
                            kind: Default::default(),
                        };

                        let worker_fn = private_ident!("_worker");

                        e.args[0].expr = Box::new(Expr::Ident(worker_fn.clone()));

                        let specifier = ImportSpecifier::Default(ImportDefaultSpecifier {
                            span: DUMMY_SP,
                            local: worker_fn,
                        });

                        self.added_imports
                            .push(ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
                                span: callee_span,
                                specifiers: vec![specifier],
                                src,
                                type_only: Default::default(),
                                asserts: Default::default(),
                            })))
                    } else {
                        // TODO(kdy1): Should we report an error?
                    }
                }
            }
            _ => {}
        }

        e
    }

    fn fold_module(&mut self, m: Module) -> Module {
        {
            let mut v = ImportAnalyzer {
                data: &mut self.data,
            };
            m.visit_with(&Invalid { span: DUMMY_SP }, &mut v);
        }

        if self.data.create_worker_factory.is_empty() {
            return m;
        }

        self.eval = Some(Evaluator::new(m.clone(), Marks::new()));

        let mut m = m.fold_children_with(self);

        prepend_stmts(&mut m.body, take(&mut self.added_imports).into_iter());

        m
    }
}

impl Visit for ImportAnalyzer<'_> {
    noop_visit_type!();

    fn visit_import_decl(&mut self, n: &ImportDecl, _: &dyn Node) {
        if &*n.src.value == "@shopify/web-worker" || &*n.src.value == "@shopify/react-web-worker" {
            for s in &n.specifiers {
                match s {
                    ImportSpecifier::Named(s) => match &s.imported {
                        Some(imported) => {
                            if &*imported.sym == "createWorkerFactory" {
                                self.data
                                    .create_worker_factory
                                    .insert(s.local.to_id(), false);
                            } else if &*imported.sym == "createPlainWorkerFactory" {
                                self.data
                                    .create_worker_factory
                                    .insert(s.local.to_id(), true);
                            }
                        }
                        None => {
                            if &*s.local.sym == "createWorkerFactory" {
                                self.data
                                    .create_worker_factory
                                    .insert(s.local.to_id(), false);
                            } else if &*s.local.sym == "createPlainWorkerFactory" {
                                self.data
                                    .create_worker_factory
                                    .insert(s.local.to_id(), true);
                            }
                        }
                    },
                    ImportSpecifier::Default(_) => {}
                    ImportSpecifier::Namespace(_) => {}
                }
            }
        }
    }
}
