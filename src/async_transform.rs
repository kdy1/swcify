use std::collections::HashMap;

use swc_common::DUMMY_SP;
use swc_ecmascript::ast::{
    ArrowExpr, AssignExpr, BindingIdent, BlockStmt, BlockStmtOrExpr, CallExpr, Expr, ExprOrSpread,
    ExprOrSuper, FnExpr, Function, Ident, ImportDecl, ImportSpecifier, KeyValueProp, Lit,
    MemberExpr, ObjectLit, Pat, Prop, PropName, PropOrSpread, ReturnStmt, Stmt, Str, StrKind, PatOrExpr,
    VarDecl,
};
use swc_ecmascript::utils::ident::{Id, IdentLike};
use swc_ecmascript::visit::{Fold, FoldWith};

pub fn async_transform() -> impl Fold {
    // Default packages to process
    let packages: HashMap<String, Vec<String>> = [
        (
            "@shopify/alpaql/async".to_string(),
            vec!["createAsyncQuery".to_string()],
        ),
        (
            "@shopify/async".to_string(),
            vec!["createResolver".to_string()],
        ),
        (
            "@shopify/react-async".to_string(),
            vec![
                "createAsyncContext".to_string(),
                "createAsyncComponent".to_string(),
            ],
        ),
        (
            "@shopify/react-graphql".to_string(),
            vec![
                "createAsyncQueryComponent".to_string(),
                "createAsyncQuery".to_string(),
            ],
        ),
    ]
    .iter()
    .cloned()
    .collect();
    let webpack = true;
    AsyncTransform {
        packages,
        webpack,
        bindings: vec![],
        overridden_bindings: vec![vec![]],
    }
}

#[derive(Debug)]
struct AsyncTransform {
    packages: HashMap<String, Vec<String>>,
    webpack: bool,
    bindings: Vec<Id>,
    overridden_bindings: Vec<Vec<Id>>,
}

impl Fold for AsyncTransform {
    fn fold_import_decl(&mut self, decl: ImportDecl) -> ImportDecl {
        let ImportDecl {
            ref src,
            ref specifiers,
            ..
        } = decl;
        if self.packages.contains_key(&src.value.to_string()) {
            for specifier in specifiers {
                match specifier {
                    ImportSpecifier::Default(default_specifier) => {
                        self.bindings.push(default_specifier.local.to_id());
                    }
                    ImportSpecifier::Named(named_specifier) => {
                        self.bindings.push(named_specifier.local.to_id())
                    }
                    _ => {}
                }
            }
        }
        decl
    }

    fn fold_block_stmt(&mut self, block: BlockStmt) -> BlockStmt {
        self.overridden_bindings.push(vec![]);
        let block = block.fold_children_with(self);
        self.overridden_bindings.pop();
        block
    }

    fn fold_assign_expr(&mut self, assign_expr: AssignExpr) -> AssignExpr {
        let assign_expr = assign_expr.fold_children_with(self);
        // Check if assignment overrides target import
        if let PatOrExpr::Pat(pattern) = &assign_expr.left {
            if let Pat::Ident(BindingIdent { id, .. }) = &**pattern {
                if self.bindings.contains(&id.to_id()) {
                    if let Some(block_overriden_bindings) = self.overridden_bindings.last_mut() {
                        block_overriden_bindings.push(id.to_id());
                    }
                }
            }
        }
        assign_expr
    }

    fn fold_var_decl(&mut self, var_decl: VarDecl) -> VarDecl {
        let var_decl = var_decl.fold_children_with(self);
        // Check if declaration overrides target import
        for decl in var_decl.decls.iter() {
        if let Pat::Ident(BindingIdent { id, .. }) = &decl.name {
                if self.bindings.contains(&id.to_id()) {
                    if let Some(block_overriden_bindings) = self.overridden_bindings.last_mut() {
                        block_overriden_bindings.push(id.to_id());
                    }
                }
            }
        }
        var_decl
    }

    fn fold_call_expr(&mut self, expr: CallExpr) -> CallExpr {
        let mut expr = expr.fold_children_with(self);

        if let ExprOrSuper::Expr(i) = &expr.callee {
            if let Expr::Ident(identifier) = &**i {
                if self.is_target_binding(&identifier.to_id()) {
                    if expr.args.len() == 1 {
                        if let Expr::Object(object_arg) = &mut *expr.args[0].expr {
                            let mut import_path: Option<String> = None;
                            for prop_spread in object_arg.props.iter() {
                                if let PropOrSpread::Prop(prop) = prop_spread {
                                    match &**prop {
                                        Prop::KeyValue(key_val) => match &key_val.key {
                                            PropName::Ident(Ident { sym: key_sym, .. }) => {
                                                if key_sym == "load" {
                                                    import_path = get_import_path_from_expr(&*key_val.value);
                                                } else if key_sym == "id" {
                                                    // do nothing when id prop already exists
                                                    break;
                                                }
                                            }
                                            _ => {}
                                        },
                                        Prop::Method(method) => {
                                            if let Some(block_stmt) = &method.function.body {
                                                import_path =
                                                    get_import_path_from_block_stmt(block_stmt);
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            if let Some(path) = import_path {
                                add_id_option(object_arg, path, self.webpack);
                            } 
                        }
                    } 
                }
            }
        }
        expr
    }
}

fn add_id_option(object: &mut ObjectLit, path: String, webpack: bool) {
    let resolve_call_ident = if webpack { "resolveWeak" } else { "resolve" };
    let req_expr = Expr::Call(CallExpr {
        span: DUMMY_SP,
        callee: ExprOrSuper::Expr(Box::new(Expr::Member(MemberExpr {
            span: DUMMY_SP,
            obj: ExprOrSuper::Expr(Box::new(Expr::Ident(Ident::new(
                "require".into(),
                DUMMY_SP,
            )))),
            prop: Box::new(Expr::Ident(Ident::new(resolve_call_ident.into(), DUMMY_SP))),
            computed: false,
        }))),
        args: vec![ExprOrSpread {
            spread: None,
            expr: Box::new(Expr::Lit(Lit::Str(Str {
                value: path.into(),
                span: DUMMY_SP,
                kind: StrKind::Synthesized {},
                has_escape: false,
            }))),
        }],
        type_args: None,
    });
    let prop_val = ArrowExpr {
        params: vec![],
        body: BlockStmtOrExpr::Expr(Box::new(req_expr)),
        is_async: false,
        is_generator: false,
        span: DUMMY_SP,
        return_type: None,
        type_params: None,
    };
    let gen_arg = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("id".into(), DUMMY_SP)),
        value: Box::new(Expr::Arrow(prop_val)),
    })));
    object.props.push(gen_arg);
}

impl AsyncTransform {
    fn is_target_binding(&mut self, id: &Id) -> bool {
        if self.bindings.contains(id){
            for block_overridden_bindings in self.overridden_bindings.iter() {
                for overridden_binding in block_overridden_bindings.iter() {
                    if overridden_binding == id {
                        return false;
                    }
                }
            }
            return true;
        }
        false
    }
}

fn get_import_path_from_expr(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Arrow(arrow_expr) => match &arrow_expr.body {
            BlockStmtOrExpr::Expr(body_expr) => {
                if let Expr::Call(call_expr) = &**body_expr {
                    return get_import_path_from_import_call(call_expr);
                }
            }
            BlockStmtOrExpr::BlockStmt(block_stmt) => {
                return get_import_path_from_block_stmt(block_stmt);
            }
        },
        Expr::Fn(FnExpr {
            function:
                Function {
                    body: Some(block_stmt),
                    ..
                },
            ..
        }) => {
            return get_import_path_from_block_stmt(block_stmt);
        }
        _ => {}
    }
    None
}

fn get_import_path_from_block_stmt(block_stmt: &BlockStmt) -> Option<String> {
    // Checks if `return import..` matches last statment
    if let Some(Stmt::Return(ReturnStmt {
        arg: Some(return_arg),
        ..
    })) = block_stmt.stmts.last()
    {
        if let Expr::Call(call_expr) = &**return_arg {
            return get_import_path_from_import_call(call_expr);
        }
    }
    None
}

fn get_import_path_from_import_call(call_expr: &CallExpr) -> Option<String> {
    if let ExprOrSuper::Expr(e) = &call_expr.callee {
        if let Expr::Ident(Ident { sym, .. }) = &**e {
            if sym == "import" {
                if let Expr::Lit(Lit::Str(Str { value, .. })) = &*call_expr.args[0].expr {
                    return Some(value.to_string());
                }
            }
        }
    }
    None
}
