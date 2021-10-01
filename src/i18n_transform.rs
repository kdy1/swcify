use radix_fmt::radix_36;
use rustc_hash::FxHasher;
use std::hash::Hasher;
use std::path::PathBuf;
use swc_common::source_map::Pos;
use swc_common::{
    comments::{Comment, CommentKind, Comments},
    BytePos, FileName, Span, DUMMY_SP,
};
use swc_ecmascript::ast::{
    ArrayLit, ArrowExpr, BinExpr, BinaryOp, BindingIdent, BlockStmt, BlockStmtOrExpr, CallExpr,
    Decl, Expr, ExprOrSpread, ExprOrSuper, Function, Ident, IfStmt, ImportDecl,
    ImportDefaultSpecifier, ImportSpecifier, KeyValueProp, Lit, MemberExpr, MethodProp, Module,
    ModuleDecl, ModuleItem, ObjectLit, Param, Pat, Prop, PropName, PropOrSpread, ReturnStmt, Stmt,
    Str, StrKind, SwitchCase, SwitchStmt, Tpl, TplElement, VarDecl, VarDeclKind, VarDeclarator,
};
use swc_ecmascript::utils::ident::{Id, IdentLike};
use swc_ecmascript::visit::{Fold, FoldWith};

const I18N_PKG_NAME: &str = "@shopify/react-i18n";
const TRANSLATION_DIRECTORY_NAME: &str = "translations";
const DEFAULT_INDEX_TRANSLATION_ARRAY_ID: &str = "__shopify__i18n_translations";
const I18N_CALL_NAMES: [&str; 2] = ["useI18n", "withI18n"];

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum I18nMode {
    WithDynamicPaths,
    WithExplicitPaths,
    FromGeneratedIndex,
    FromDictionaryIndex,
}

pub struct I18nTransform<'a> {
    filename: PathBuf,
    mode: I18nMode,
    default_locale: String,
    comments: &'a dyn Comments,
    translation_file_paths: Option<Vec<PathBuf>>,
    bindings: Vec<Id>,
    call_rewritten: bool,
}

pub fn i18n_transform<'a>(
    filename: FileName,
    mode: I18nMode,
    default_locale: String,
    comments: &'a dyn Comments,
) -> I18nTransform<'a> {
    let filename = match filename {
        FileName::Real(pathbuf) => pathbuf,
        _ => panic!("Unhandled filename type."),
    };

    let translation_file_paths: Option<Vec<PathBuf>> =
        get_translation_file_paths(&filename, &String::from("translations"));
    I18nTransform {
        filename,
        mode,
        default_locale,
        comments,
        translation_file_paths,
        bindings: vec![],
        call_rewritten: false,
    }
}

impl I18nTransform<'_> {
    fn inject_with_i18n_arguments(&mut self, call_expr: &mut CallExpr) {
        let id = generate_id(&self.filename);
        let translation_file_paths = match &self.translation_file_paths {
            Some(translation_file_paths) => translation_file_paths.clone(),
            None => return,
        };

        // Comments can only be mapped to nodes with non dummy spans
        // The .lo portion is set to 1 byte after the callexpr lo
        // The .hi portion must be higher than lo, but doesn't have to be accurate.
        let comment_span_lo = call_expr.span.lo() + BytePos(1);

        let i18n_args: Expr = match self.mode {
            I18nMode::WithDynamicPaths => {
                let fallback_expr = fallback_expr_from_locale(&self.default_locale);
                let locale_ids = get_locale_ids(&translation_file_paths, &self.default_locale);
                let check_stmt = translation_fn_check(is_in_str_array(&locale_ids));
                let comment_span = add_leading_comment(
                    &mut self.comments,
                    comment_span_lo,
                    format!(
                        " webpackChunkName: \"{}-i18n\", webpackMode: \"lazy-once\" ",
                        id
                    ),
                );
                let return_stmt =
                    import_promise_return_stmt(None, default_dict_arrow_fn(), comment_span);
                let translation_fn_stmts = vec![check_stmt, return_stmt];

                generate_i18n_call_arguments(id, fallback_expr, translation_fn_stmts)
            }
            I18nMode::WithExplicitPaths => {
                let fallback_expr = fallback_expr_from_locale(&self.default_locale);
                let locale_ids = get_locale_ids(&translation_file_paths, &self.default_locale);
                let mut translation_fn_stmts = vec![];
                match locale_ids.len() {
                    num_locals if num_locals == 0 => {
                        translation_fn_stmts.push(empty_return_stmt());
                    }
                    num_locals if num_locals == 1 => {
                        translation_fn_stmts
                            .push(translation_fn_check(locale_eq_str_expr(&locale_ids[0])));

                        let comment_span = add_leading_comment(
                            &mut self.comments,
                            comment_span_lo,
                            format!(" webpackChunkName: \"{}-i18n\" ", id),
                        );
                        translation_fn_stmts.push(import_promise_return_stmt(
                            Some(locale_ids[0].clone()),
                            default_dict_arrow_fn(),
                            comment_span,
                        ));
                    }
                    num_locals @ _ => {
                        translation_fn_stmts.push(explicit_paths_define_arrow_fn_stmt());
                        for i in 0..num_locals {
                            add_leading_comment(
                                &mut self.comments,
                                comment_span_lo + BytePos::from_usize(i),
                                format!(" webpackChunkName: \"{}-i18n\" ", id),
                            );
                        }
                        translation_fn_stmts
                            .push(explicit_paths_switch_stmt(&locale_ids, comment_span_lo));
                    }
                };
                generate_i18n_call_arguments(id, fallback_expr, translation_fn_stmts)
            }
            I18nMode::FromGeneratedIndex => {
                let fallback_expr = fallback_expr_from_locale(&self.default_locale);
                let check_stmt = translation_fn_check(is_in_array_var(String::from(
                    DEFAULT_INDEX_TRANSLATION_ARRAY_ID,
                )));
                let comment_span = add_leading_comment(
                    &mut self.comments,
                    comment_span_lo,
                    format!(
                        " webpackChunkName: \"{}-i18n\", webpackMode: \"lazy-once\" ",
                        id
                    ),
                );
                let return_stmt =
                    import_promise_return_stmt(None, default_dict_arrow_fn(), comment_span);
                let translation_fn_stmts = vec![check_stmt, return_stmt];

                generate_i18n_call_arguments(id, fallback_expr, translation_fn_stmts)
            }
            I18nMode::FromDictionaryIndex => {
                let fallback_expr = fallback_expr_from_dictionary(&String::from(
                    DEFAULT_INDEX_TRANSLATION_ARRAY_ID,
                ));
                let return_stmt =
                    dictionary_index_return_stmt(&String::from(DEFAULT_INDEX_TRANSLATION_ARRAY_ID));
                let translation_fn_stmts = vec![return_stmt];

                generate_i18n_call_arguments(id, fallback_expr, translation_fn_stmts)
            }
        };
        call_expr.args.push(ExprOrSpread {
            expr: Box::new(i18n_args),
            spread: None,
        });
    }
}

impl Fold for I18nTransform<'_> {
    fn fold_module(&mut self, module: Module) -> Module {
        // skip transform if translation files not found
        if self.translation_file_paths == None {
            return module;
        }
        let mut module = module.fold_children_with(self);

        if self.bindings.len() > 0 && self.call_rewritten {
            let (import_id, import_src) = match self.mode {
                I18nMode::FromGeneratedIndex | I18nMode::FromDictionaryIndex => (
                    String::from(DEFAULT_INDEX_TRANSLATION_ARRAY_ID),
                    format!("./{}", TRANSLATION_DIRECTORY_NAME),
                ),
                _ => (
                    get_locale_id(&self.default_locale),
                    format!(
                        "./{}/{}.json",
                        TRANSLATION_DIRECTORY_NAME, self.default_locale
                    ),
                ),
            };
            insert_import(&mut module, &import_id, &import_src);
        }
        module
    }

    fn fold_import_decl(&mut self, decl: ImportDecl) -> ImportDecl {
        let ImportDecl {
            ref src,
            ref specifiers,
            ..
        } = decl;
        if src.value.to_string() == I18N_PKG_NAME {
            for specifier in specifiers {
                match specifier {
                    ImportSpecifier::Default(default_specifier) => {
                        if I18N_CALL_NAMES
                            .iter()
                            .any(|&name| name == &default_specifier.local.sym)
                        {
                            self.bindings.push(default_specifier.local.to_id());
                        }
                    }
                    ImportSpecifier::Named(named_specifier) => {
                        if let Some(imported) = &named_specifier.imported {
                            if I18N_CALL_NAMES.iter().any(|&name| name == &imported.sym) {
                                self.bindings.push(named_specifier.local.to_id())
                            }
                        } else {
                            if I18N_CALL_NAMES
                                .iter()
                                .any(|&name| name == &named_specifier.local.sym)
                            {
                                self.bindings.push(named_specifier.local.to_id())
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        decl
    }

    fn fold_call_expr(&mut self, expr: CallExpr) -> CallExpr {
        let mut expr = expr.fold_children_with(self);
        if expr.args.len() == 0 {
            if let ExprOrSuper::Expr(i) = &expr.callee {
                if let Expr::Ident(identifier) = &**i {
                    if self.bindings.contains(&identifier.to_id()) {
                        if self.call_rewritten {
                            panic!("You attempted to use bindingName referencePathsToRewrite.length times in a single file. This is not supported by the Babel plugin that automatically inserts translations.")
                        } else {
                            self.call_rewritten = true;
                        }
                        if expr.args.len() != 0 {
                            panic!("No translation files found in translations/ folder.");
                        }
                        self.inject_with_i18n_arguments(&mut expr);
                    }
                }
            }
        }
        expr
    }
}

// e.g., import import_id from "import_src";
fn insert_import(module: &mut Module, import_id: &String, import_src: &String) {
    let import_decl = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
        span: DUMMY_SP,
        specifiers: vec![ImportSpecifier::Default(ImportDefaultSpecifier {
            span: DUMMY_SP,
            local: Ident::new(import_id.clone().into(), DUMMY_SP),
        })],
        src: Str {
            value: import_src.clone().into(),
            span: DUMMY_SP,
            kind: StrKind::Synthesized,
            has_escape: false,
        },
        type_only: false,
        asserts: None,
    }));
    module.body.insert(0, import_decl);
}

// e.g., {id: [id], fallback: "_fallback_val", translations (locale) {fn_stmts...}}
fn generate_i18n_call_arguments(
    id: String,
    fallback_val: Box<Expr>,
    translation_fn_stmts: Vec<Stmt>,
) -> Expr {
    let id_prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("id".into(), DUMMY_SP)),
        value: Box::new(Expr::from(Str {
            value: id.into(),
            span: DUMMY_SP,
            kind: StrKind::Synthesized {},
            has_escape: false,
        })),
    })));
    let fallback_prop = PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
        key: PropName::Ident(Ident::new("fallback".into(), DUMMY_SP)),
        value: fallback_val,
    })));
    let tranlation_fn_params = vec![Param {
        span: DUMMY_SP,
        decorators: vec![],
        pat: Pat::Ident(BindingIdent::from(Ident::new("locale".into(), DUMMY_SP))),
    }];

    let translation_fn_body = BlockStmt {
        span: DUMMY_SP,
        stmts: translation_fn_stmts,
    };

    let tranlations_prop = PropOrSpread::Prop(Box::new(Prop::Method(MethodProp {
        key: PropName::Ident(Ident::new("translations".into(), DUMMY_SP)),
        function: Function {
            params: tranlation_fn_params,
            decorators: vec![],
            span: DUMMY_SP,
            is_generator: false,
            is_async: false,
            type_params: None,
            return_type: None,
            body: Some(translation_fn_body),
        },
    })));

    Expr::Object(ObjectLit {
        span: DUMMY_SP,
        props: vec![id_prop, fallback_prop, tranlations_prop],
    })
}

// e.g.: '_en'
fn fallback_expr_from_locale(locale: &String) -> Box<Expr> {
    Box::new(Expr::Ident(Ident::new(
        get_locale_id(locale).into(),
        DUMMY_SP,
    )))
}

// e.g.: Object.values(__shopify__i18n_translations)[0]
fn fallback_expr_from_dictionary(dict_id: &String) -> Box<Expr> {
    Box::new(Expr::Member(MemberExpr {
        span: DUMMY_SP,
        prop: Box::new(Expr::from(0f64)),
        computed: true,
        obj: ExprOrSuper::Expr(Box::new(Expr::Call(CallExpr {
            span: DUMMY_SP,
            callee: ExprOrSuper::Expr(Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: ExprOrSuper::Expr(Box::new(Expr::Ident(Ident::new(
                    "Object".into(),
                    DUMMY_SP,
                )))),
                prop: Box::new(Expr::Ident(Ident::new("values".into(), DUMMY_SP))),
                computed: false,
            }))),
            args: vec![ExprOrSpread {
                spread: None,
                expr: Box::new(Expr::Ident(Ident::new(dict_id.as_str().into(), DUMMY_SP))),
            }],
            type_args: None,
        }))),
    }))
}

// e.g.: locale !== "str_val"
fn locale_eq_str_expr(str_val: &String) -> Box<Expr> {
    Box::new(Expr::Bin(BinExpr {
        span: DUMMY_SP,
        op: BinaryOp::NotEqEq,
        left: Box::new(Expr::Ident(Ident::new("locale".into(), DUMMY_SP))),
        right: Box::new(Expr::Lit(Lit::Str(Str {
            value: str_val.as_str().into(),
            span: DUMMY_SP,
            kind: StrKind::Synthesized {},
            has_escape: false,
        }))),
    }))
}

// e.g.: ["de", "fr", "zh-TW"].indexOf(locale) < 0
fn is_in_str_array(locale_ids: &Vec<String>) -> Box<Expr> {
    let locale_ids_array = locale_ids
        .iter()
        .map(|id| {
            Some(ExprOrSpread {
                spread: None,
                expr: Box::new(Expr::Lit(Lit::Str(Str {
                    value: id.as_str().into(),
                    span: DUMMY_SP,
                    kind: StrKind::Synthesized {},
                    has_escape: false,
                }))),
            })
        })
        .collect();

    Box::new(Expr::Bin(BinExpr {
        span: DUMMY_SP,
        op: BinaryOp::Lt,
        left: Box::new(Expr::Call(CallExpr {
            span: DUMMY_SP,
            callee: ExprOrSuper::Expr(Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: ExprOrSuper::Expr(Box::new(Expr::Array(ArrayLit {
                    span: DUMMY_SP,
                    elems: locale_ids_array,
                }))),
                prop: Box::new(Expr::Ident(Ident::new("indexOf".into(), DUMMY_SP))),
                computed: false,
            }))),
            args: vec![ExprOrSpread {
                spread: None,
                expr: Box::new(Expr::Ident(Ident::new("locale".into(), DUMMY_SP))),
            }],
            type_args: None,
        })),
        right: Box::new(Expr::from(0f64)),
    }))
}

// e.g.: var_id.indexOf(locale) < 0
fn is_in_array_var(var_id: String) -> Box<Expr> {
    Box::new(Expr::Bin(BinExpr {
        span: DUMMY_SP,
        op: BinaryOp::Lt,
        left: Box::new(Expr::Call(CallExpr {
            span: DUMMY_SP,
            callee: ExprOrSuper::Expr(Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: ExprOrSuper::Expr(Box::new(Expr::Ident(Ident::new(var_id.into(), DUMMY_SP)))),
                prop: Box::new(Expr::Ident(Ident::new("indexOf".into(), DUMMY_SP))),
                computed: false,
            }))),
            args: vec![ExprOrSpread {
                spread: None,
                expr: Box::new(Expr::Ident(Ident::new("locale".into(), DUMMY_SP))),
            }],
            type_args: None,
        })),
        right: Box::new(Expr::from(0f64)),
    }))
}

// e.g.: if([test]){return;}
fn translation_fn_check(test: Box<Expr>) -> Stmt {
    Stmt::If(IfStmt {
        span: DUMMY_SP,
        test,
        cons: Box::new(Stmt::Block(BlockStmt {
            span: DUMMY_SP,
            stmts: vec![Stmt::Return(ReturnStmt {
                span: DUMMY_SP,
                arg: None,
            })],
        })),
        alt: None,
    })
}

// e.g.: return import(`./translations/${locale}.json`).then((dict) => dict && dict.default);
fn import_promise_return_stmt(
    import_arg_locale: Option<String>,
    on_resolve: Expr,
    comment_span: Span,
) -> Stmt {
    let import_arg = match import_arg_locale {
        Some(locale) => Box::new(Expr::Lit(Lit::Str(Str {
            value: format!("./{}/{}.json", TRANSLATION_DIRECTORY_NAME, locale)
                .as_str()
                .into(),
            span: comment_span,
            kind: StrKind::Synthesized {},
            has_escape: false,
        }))),
        None => Box::new(Expr::Tpl(Tpl {
            span: comment_span,
            exprs: vec![],
            quasis: vec![TplElement {
                span: DUMMY_SP,
                tail: false,
                cooked: None,
                raw: Str {
                    value: "./translations/${locale}.json".into(),
                    span: DUMMY_SP,
                    kind: StrKind::Synthesized,
                    has_escape: false,
                },
            }],
        })),
    };

    Stmt::Return(ReturnStmt {
        span: DUMMY_SP,
        arg: Some(Box::new(Expr::Call(CallExpr {
            span: DUMMY_SP,
            args: vec![ExprOrSpread {
                spread: None,
                expr: Box::new(on_resolve),
            }],
            type_args: None,
            callee: ExprOrSuper::Expr(Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: ExprOrSuper::Expr(Box::new(Expr::Call(CallExpr {
                    span: DUMMY_SP,
                    callee: ExprOrSuper::Expr(Box::new(Expr::Ident(Ident::new(
                        "import".into(),
                        DUMMY_SP,
                    )))),
                    args: vec![ExprOrSpread {
                        spread: None,
                        expr: import_arg,
                    }],
                    type_args: None,
                }))),
                prop: Box::new(Expr::Ident(Ident::new("then".into(), DUMMY_SP))),
                computed: false,
            }))),
        }))),
    })
}

// e.g.: return;
fn empty_return_stmt() -> Stmt {
    Stmt::Return(ReturnStmt {
        span: DUMMY_SP,
        arg: None,
    })
}

fn explicit_paths_define_arrow_fn_stmt() -> Stmt {
    Stmt::Decl(Decl::Var(VarDecl {
        span: DUMMY_SP,
        kind: VarDeclKind::Const,
        declare: false,
        decls: vec![VarDeclarator {
            span: DUMMY_SP,
            name: Pat::Ident(BindingIdent::from(Ident::new(
                "returnDefault".into(),
                DUMMY_SP,
            ))),
            definite: true,
            init: Some(Box::new(default_dict_arrow_fn())),
        }],
    }))
}

// e.g.: (dict) => dict && dict.default
fn default_dict_arrow_fn() -> Expr {
    Expr::Arrow(ArrowExpr {
        span: DUMMY_SP,
        params: vec![Pat::Ident(BindingIdent::from(Ident::new(
            "dict".into(),
            DUMMY_SP,
        )))],
        body: BlockStmtOrExpr::Expr(Box::new(Expr::Bin(BinExpr {
            span: DUMMY_SP,
            op: BinaryOp::LogicalAnd,
            left: Box::new(Expr::Ident(Ident::new("dict".into(), DUMMY_SP))),
            right: Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: ExprOrSuper::Expr(Box::new(Expr::Ident(Ident::new("dict".into(), DUMMY_SP)))),
                prop: Box::new(Expr::Ident(Ident::new("default".into(), DUMMY_SP))),
                computed: false,
            })),
        }))),
        is_async: false,
        is_generator: false,
        return_type: None,
        type_params: None,
    })
}

// e.g.: switch (locale) { case "de": return import("./translations/de.json").then(returnDefault); ,...
fn explicit_paths_switch_stmt(locale_ids: &Vec<String>, span_lo: BytePos) -> Stmt {
    let cases: Vec<_> = locale_ids
        .iter()
        .enumerate()
        .map(|(i, id)| {
            let comment_span = Span {
                lo: span_lo + BytePos::from_usize(i),
                hi: span_lo + BytePos::from_usize(i + 1),
                ctxt: Default::default(),
            };
            SwitchCase {
                span: DUMMY_SP,
                test: Some(Box::new(Expr::Lit(Lit::Str(Str {
                    value: id.as_str().into(),
                    span: DUMMY_SP,
                    kind: StrKind::Synthesized {},
                    has_escape: false,
                })))),
                cons: vec![import_promise_return_stmt(
                    Some(id.to_string()),
                    Expr::Ident(Ident::new("returnDefault".into(), DUMMY_SP)),
                    comment_span,
                )],
            }
        })
        .collect();
    Stmt::Switch(SwitchStmt {
        span: DUMMY_SP,
        discriminant: Box::new(Expr::Ident(Ident::new("locale".into(), DUMMY_SP))),
        cases,
    })
}

// e.g.: Promise.resolve(__shopify__i18n_translations[locale])
fn dictionary_index_return_stmt(dict_id: &String) -> Stmt {
    Stmt::Return(ReturnStmt {
        span: DUMMY_SP,
        arg: Some(Box::new(Expr::Call(CallExpr {
            span: DUMMY_SP,
            callee: ExprOrSuper::Expr(Box::new(Expr::Member(MemberExpr {
                span: DUMMY_SP,
                obj: ExprOrSuper::Expr(Box::new(Expr::Ident(Ident::new(
                    "Promise".into(),
                    DUMMY_SP,
                )))),
                prop: Box::new(Expr::Ident(Ident::new("resolve".into(), DUMMY_SP))),
                computed: false,
            }))),
            args: vec![ExprOrSpread {
                spread: None,
                expr: Box::new(Expr::Member(MemberExpr {
                    span: DUMMY_SP,
                    computed: true,
                    obj: ExprOrSuper::Expr(Box::new(Expr::Ident(Ident::new(
                        dict_id.as_str().into(),
                        DUMMY_SP,
                    )))),
                    prop: Box::new(Expr::Ident(Ident::new("locale".into(), DUMMY_SP))),
                })),
            }],
            type_args: None,
        }))),
    })
}

fn generate_id(filename: &PathBuf) -> String {
    let mut hasher = FxHasher::default();
    hasher.write(
        filename
            .to_str()
            .expect("Failed to convert filename to string.")
            .as_bytes(),
    );
    let hash = radix_36(hasher.finish());
    let legible: &str = filename
        .file_stem()
        .expect("Failed to get file stem from filename.")
        .to_str()
        .expect("Failed to convert filename to string.");
    return format!("{}_{}", legible, hash);
}

fn get_translation_file_paths(
    filename: &PathBuf,
    translation_dir_name: &String,
) -> Option<Vec<PathBuf>> {
    let mut translation_dir: PathBuf = match filename.parent() {
        Some(path) => PathBuf::from(path),
        None => {
            panic!("Parent directory not found");
        }
    };
    translation_dir.push(translation_dir_name);
    if !translation_dir.is_dir() {
        return None;
    }
    let read_dir = match translation_dir.read_dir() {
        Ok(read_dir) => read_dir,
        Err(_) => return None,
    };
    let mut translation_files: Vec<PathBuf> = read_dir
        .map(|entry| {
            let entry = entry.unwrap();
            entry.path()
        })
        .collect();

    translation_files.retain(|item| {
        if let Some(ext) = item.extension() {
            ext == "json"
        } else {
            false
        }
    });
    if translation_files.len() == 0 {
        None
    } else {
        Some(translation_files)
    }
}

fn get_locale_ids(translation_file_paths: &Vec<PathBuf>, fallback_locale: &String) -> Vec<String> {
    let mut locale_ids: Vec<String> = translation_file_paths
        .into_iter()
        .map(|path| String::from(path.file_stem().unwrap().to_str().unwrap()))
        .collect();
    locale_ids.retain(|locale| locale != fallback_locale);
    locale_ids.sort();
    locale_ids
}

fn get_locale_id(fallback_locale: &String) -> String {
    format!("_{}", fallback_locale)
}

// adds a leading comment to provided comment map and returns a span for the node the comment should precede.
fn add_leading_comment(comments: &mut dyn Comments, pos: BytePos, text: String) -> Span {
    comments.add_leading(
        pos,
        Comment {
            span: DUMMY_SP,
            kind: CommentKind::Block,
            text,
        },
    );
    Span {
        lo: pos,
        hi: pos + BytePos(1),
        ctxt: Default::default(),
    }
}
