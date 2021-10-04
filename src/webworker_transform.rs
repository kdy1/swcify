use swc_common::DUMMY_SP;
use swc_ecmascript::ast::{CallExpr, ImportDecl};
use swc_ecmascript::visit::{Fold, FoldWith};

#[derive(Debug)]
pub struct WebWorkerTransform {}

impl WebWorkerTransform {
    pub fn with_defaults() -> Self {
        Self {}
    }
}

impl Fold for WebWorkerTransform {
    fn fold_import_decl(&mut self, decl: ImportDecl) -> ImportDecl {
        let decl = decl.fold_children_with(self);
        decl
    }

    fn fold_call_expr(&mut self, call_expr: CallExpr) -> CallExpr {
        let call_expr = call_expr.fold_children_with(self);
        call_expr
    }
}
