use rustc_hash::FxHashSet;
use swc_atoms::js_word;
use swc_common::DUMMY_SP;
use swc_ecmascript::{
    ast::*,
    minifier::{
        eval::{EvalResult, Evaluator},
        marks::Marks,
    },
    utils::{ident::IdentLike, ExprExt, Id},
    visit::{Fold, FoldWith, Node, Visit, VisitWith},
};

pub struct WebWorker {
    data: Data,
    eval: Evaluator,
}

struct ImportAnalyzer<'a> {
    data: &'a mut Data,
}

#[derive(Default)]
struct Data {
    /// All import specifiers of `createWorkerFactory`.
    create_worker_factory: FxHashSet<Id>,
}

impl Fold for WebWorker {
    // TODO(kdy1): Apply this after https://github.com/swc-project/swc/pull/2347 is merged
    // noop_fold_type!()

    fn fold_call_expr(&mut self, e: CallExpr) -> CallExpr {
        let e = e.fold_children_with(self);

        match &e.callee {
            ExprOrSuper::Expr(callee) => match &**callee {
                Expr::Ident(callee) => {
                    // This is a call to createWorkerFactory
                    if self.data.create_worker_factory.contains(&callee.to_id()) {
                        if e.args.len() == 1 && e.args[0].spread.is_none() {
                            match &*e.args[0].expr {
                                Expr::Arrow(ArrowExpr {
                                    params,
                                    body: BlockStmtOrExpr::Expr(body),
                                    is_async: false,
                                    is_generator: false,
                                    ..
                                }) if params.is_empty() => match &**body {
                                    Expr::Call(CallExpr {
                                        callee: ExprOrSuper::Expr(callee),
                                        args,
                                        ..
                                    }) => {
                                        if callee.is_ident_ref_to(js_word!("import")) {
                                            if args.len() == 1 && args[0].spread.is_none() {
                                                match self.eval.eval(&args[0].expr) {
                                                    Some(EvalResult::Lit(Lit::Str(s))) => {
                                                        // TODO(kdy1):
                                                        //
                                                        // import workerStuff from '@shopify/web-worker/webpack-loader!./worker';
                                                        // createWorkerFactory(workerStuff);
                                                    }
                                                    _ => {
                                                        // TODO(kdy1): Should we report an error?
                                                    }
                                                }
                                            } else {
                                                // TODO(kdy1): Should we report an error?
                                            }
                                        }
                                    }

                                    _ => {}
                                },

                                _ => {}
                            }
                        } else {
                            // TODO(kdy1): Should we report an error?
                        }
                    }
                }

                _ => {}
            },
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

        self.eval = Evaluator::new(m.clone(), Marks::new());

        m.fold_children_with(self)
    }
}

impl Visit for ImportAnalyzer<'_> {
    // TODO(kdy1): Apply this after https://github.com/swc-project/swc/pull/2347 is merged
    // noop_visit_type!()

    fn visit_import_decl(&mut self, n: &ImportDecl, _: &dyn Node) {
        if &*n.src.value == "@shopify/web-worker" {
            for s in &n.specifiers {
                match s {
                    ImportSpecifier::Named(s) => match &s.imported {
                        Some(imported) => {
                            if &*imported.sym == "createWorkerFactory" {
                                self.data.create_worker_factory.insert(s.local.to_id());
                            }
                        }
                        None => {
                            if &*s.local.sym == "createWorkerFactory" {
                                self.data.create_worker_factory.insert(s.local.to_id());
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
