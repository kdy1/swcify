use rustc_hash::FxHashSet;
use swc_common::DUMMY_SP;
use swc_ecmascript::{
    ast::*,
    utils::{ident::IdentLike, Id},
    visit::{Fold, FoldWith, Node, Visit, VisitWith},
};

pub struct WebWorker {
    data: Data,
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

    // fold_script can be implemented using same code, but it seems not neccessary for shopify.

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
