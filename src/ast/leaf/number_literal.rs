use crate::ast::ast_leaf::AstLeaf;
use crate::eval::eval::EvalRes;
use crate::{ast_leaf_default_eval_impl, ast_leaf_default_impl, ast_leaf_default_new, ast_leaf_factory_default_impl};

pub struct NumberLiteral {
    ast_leaf: AstLeaf,
}

impl NumberLiteral {
    ast_leaf_default_new! {NumberLiteral,NUMBER }
}

ast_leaf_default_impl! {NumberLiteral,TokenNumber}

ast_leaf_factory_default_impl! {NumberLiteralFactory,NumberLiteral}

ast_leaf_default_eval_impl! { NumberLiteral, NUMBER, NUMBER }