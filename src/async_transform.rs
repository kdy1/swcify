use std::collections::HashMap;

use swc_common::DUMMY_SP;
use swc_ecmascript::ast::{
    ArrowExpr, AssignExpr, BindingIdent, BlockStmt, BlockStmtOrExpr, CallExpr, Expr, ExprOrSpread,
    ExprOrSuper, FnExpr, Function, Ident, ImportDecl, ImportSpecifier, KeyValueProp, Lit,
    MemberExpr, ObjectLit, Pat, PatOrExpr, Prop, PropName, PropOrSpread, ReturnStmt, Stmt, Str,
    StrKind, VarDecl,
};
use swc_ecmascript::utils::ident::{Id, IdentLike};
use swc_ecmascript::visit::{Fold, FoldWith};
use maplit::hashmap;

#[derive(Debug)]
pub struct AsyncTransform {
    packages: HashMap<String, Vec<String>>,
    webpack: bool,
    bindings: Vec<Id>,
    overridden_bindings: Vec<Vec<Id>>,
}

impl AsyncTransform {
    pub fn with_defaults() -> Self {
        Self {
            packages: hashmap! {
                String::from("@shopify/alpaql/async") => vec![String::from("createAsyncQuery")],
                String::from("@shopify/async") => vec![String::from("createResolver")],
                String::from("@shopify/react-async") => vec![String::from("createAsyncContext"), String::from("createAsyncComponent")],
                String::from("@shopify/react-graphql") => vec![String::from("createAsyncQueryComponent"), String::from("createAsyncQuery")],
            },
            webpack: true,
            bindings: vec![],
            overridden_bindings: vec![vec![]],
        }
    }

    fn is_target_binding(&mut self, id: &Id) -> bool {
        if !self.bindings.contains(id) {
            return false;
        }
        !self.overridden_bindings
            .iter()
            .any(|block_overridden_bindings| {
                block_overridden_bindings
                    .iter()
                    .any(|binding| binding == id)
            })
    }
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
                        self.bindings.push(named_specifier.local.to_id());
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
        if let PatOrExpr::Pat(pattern) = assign_expr.left.clone() {
            if let Pat::Ident(BindingIdent { id, .. }) = &*pattern {
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

    fn fold_call_expr(&mut self, call_expr: CallExpr) -> CallExpr {
        let mut call_expr = call_expr.fold_children_with(self);
        if let ExprOrSuper::Expr(i) = call_expr.callee.clone() {
            if let Expr::Ident(identifier) = &*i {
                if self.is_target_binding(&identifier.to_id()) {
                    rewrite_call_expr(&mut call_expr, self.webpack);
                }
            }
        }
        call_expr
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

fn rewrite_call_expr(call_expr: &mut CallExpr, webpack: bool) -> () {
    if call_expr.args.len() == 0 {
        return;
    }
    if let Expr::Object(object_arg) = &mut *call_expr.args[0].expr {
        let mut import_path: Option<String> = None;
        for prop_spread in object_arg.props.iter() {
            if let PropOrSpread::Prop(prop) = prop_spread.clone() {
                match *prop {
                    Prop::KeyValue(key_val) => {
                        match key_val.key {
                            PropName::Ident(Ident { sym: key_sym, .. }) => {
                                if &key_sym == "load" {
                                    import_path = match *key_val.value {
                                        Expr::Arrow(arrow_expr) => {
                                            get_import_path_from_arrow_expr(arrow_expr)
                                        }
                                        Expr::Fn(fn_expr) => {
                                            get_import_path_from_function_expr(fn_expr.clone())
                                        }
                                        _ => None,
                                    };
                                } else if &key_sym == "id" {
                                    // do nothing when id prop already exists
                                    return ();
                                }
                            }
                            _ => return,
                        }
                    }
                    Prop::Method(method) => {
                        if let Some(block_stmt) = method.function.body {
                            import_path = get_import_path_from_block_stmt(block_stmt);
                        }
                    }
                    _ => return,
                }
            }
        }
        if let Some(path) = import_path {
            add_id_option(object_arg, path, webpack);
            return ();
        }
    }
}

fn get_import_path_from_arrow_expr(arrow_expr: ArrowExpr) -> Option<String> {
    match arrow_expr.body {
        BlockStmtOrExpr::Expr(body_expr) => {
            if let Expr::Call(call_expr) = *body_expr {
                return get_import_path_from_import_call(call_expr);
            }
        }
        BlockStmtOrExpr::BlockStmt(block_stmt) => {
            return get_import_path_from_block_stmt(block_stmt);
        }
    }
    None
}

fn get_import_path_from_function_expr(fn_expr: FnExpr) -> Option<String> {
    if let FnExpr {
        function: Function {
            body: Some(block_stmt),
            ..
        },
        ..
    } = fn_expr
    {
        return get_import_path_from_block_stmt(block_stmt);
    }
    None
}

fn get_import_path_from_block_stmt(block_stmt: BlockStmt) -> Option<String> {
    // Checks if `return import..` matches last statment
    if let Some(Stmt::Return(ReturnStmt {
        arg: Some(return_arg),
        ..
    })) = &block_stmt.stmts.last()
    {
        if let Expr::Call(call_expr) = *return_arg.clone() {
            return get_import_path_from_import_call(call_expr);
        }
    }
    None
}

fn get_import_path_from_import_call(call_expr: CallExpr) -> Option<String> {
    if let ExprOrSuper::Expr(e) = call_expr.callee {
        if let Expr::Ident(Ident { sym, .. }) = *e {
            if &sym == "import" {
                if let Expr::Lit(Lit::Str(Str { value, .. })) = *call_expr.args[0].expr.clone() {
                    return Some(value.to_string());
                }
            }
        }
    }
    None
}
