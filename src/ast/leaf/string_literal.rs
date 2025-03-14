use crate::ast::ast_leaf::AstLeaf;
use crate::eval::eval::EvalRes;
use crate::{ast_leaf_default_eval_impl, ast_leaf_default_impl, ast_leaf_default_new, ast_leaf_factory_default_impl};

#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral {
    ast_leaf: AstLeaf,
}

impl StringLiteral {
    ast_leaf_default_new! {StringLiteral,StringVal }
}

ast_leaf_default_impl! {StringLiteral,TokenText}

ast_leaf_factory_default_impl! {StringLiteralFactory,StringLiteral}

ast_leaf_default_eval_impl! { StringLiteral, StringVal, StringVal }
