use std::collections::HashMap;

use swc_common::DUMMY_SP;
use swc_ecmascript::ast::{
    ArrowExpr, BlockStmtOrExpr, CallExpr, Expr, ExprOrSpread, ExprOrSuper, Ident, ImportDecl,
    ImportSpecifier, KeyValueProp, Lit, MemberExpr, ObjectLit, Prop, PropName, PropOrSpread, Str,
    StrKind,
};
use swc_ecmascript::utils::ident::{Id, IdentLike};
use swc_ecmascript::visit::{Fold, FoldWith};

pub fn async_transform() -> impl Fold {
    // TODO: take non default packages as option
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
    // TODO: take webpack bool as option
    let webpack = true;
    AsyncTransform {
        packages,
        webpack,
        bindings: vec![],
    }
}

#[derive(Debug)]
struct AsyncTransform {
    packages: HashMap<String, Vec<String>>,
    webpack: bool,
    bindings: Vec<Id>,
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
                    // ImportSpecifier::Named(named_specifier) => {
                    //     // TODO: handle named imports
                    // }
                    _ => {}
                }
            }
        }
        decl
    }

    fn fold_call_expr(&mut self, expr: CallExpr) -> CallExpr {
        let mut expr = expr.fold_children_with(self);

        if let ExprOrSuper::Expr(i) = &expr.callee {
            if let Expr::Ident(identifier) = &**i {
                println!("id: {}", &identifier);
                if self.bindings.contains(&identifier.to_id()) {
                    // TODO: emit error if invalid arg lengths
                    if expr.args.len() == 1 {
                        if let Expr::Object(object_arg) = &mut *expr.args[0].expr {
                            let mut import_path: Option<String> = None;
                            for prop_spread in object_arg.props.iter() {
                                if let PropOrSpread::Prop(prop) = prop_spread {
                                    match &**prop {
                                        Prop::KeyValue(key_val) => match &key_val.key {
                                            PropName::Ident(Ident { sym: key_sym, .. }) => {
                                                println!("Oof: {}", key_sym);
                                                if key_sym == "load" {
                                                    import_path = get_import_path(&key_val.value);
                                                } else if key_sym == "id" {
                                                    // do nothing when id prop already exists
                                                    break;
                                                }
                                            }
                                            _ => {}
                                        },
                                        // Prop::Method(method) => {
                                        //     // TODO: get path from load()
                                        // }
                                        _ => {}
                                    }
                                }
                            }
                            match import_path {
                                Some(path) => {
                                    add_id_option(object_arg, path, self.webpack);
                                }
                                // TODO: handle import path not found
                                None => {}
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

fn get_import_path(expr: &Box<Expr>) -> Option<String> {
    match &**expr {
        Expr::Arrow(arrow_expr) => {
            match &arrow_expr.body {
                BlockStmtOrExpr::Expr(body_expr) => {
                    if let Expr::Call(call_expr) = &**body_expr {
                        return get_import_path_from_import_call(call_expr);
                    }
                },
                BlockStmtOrExpr::BlockStmt(_block_stmt) => {
                    // TODO: handle fn expression
                }
            }
        }
        // Expr::Fn(fn_expr) => {
        //     // TODO: fn expression
        // }
        _ => {}
    }
    None
}

fn get_import_path_from_import_call(call_expr: &CallExpr) -> Option<String> {
    if let ExprOrSuper::Expr(e) = &call_expr.callee {
        if let Expr::Ident(Ident { sym, .. }) = &**e {
            if sym == "import" {
                if call_expr.args.len() == 0 {
                    // TODO: handle empty string
                } else if let Expr::Lit(Lit::Str(Str { value, .. })) = &*call_expr.args[0].expr {
                    println!("import??{}", value);
                    return Some(value.to_string());
                }
            }
        }
    }
    None
}
