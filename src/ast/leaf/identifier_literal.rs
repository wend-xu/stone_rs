use crate::ast::ast_leaf::AstLeaf;
use crate::eval::eval::EvalRes;
use crate::{ast_leaf_default_eval_impl, ast_leaf_default_impl, ast_leaf_default_new, ast_leaf_factory_default_impl};

pub struct IdentifierLiteral {
    ast_leaf: AstLeaf,
}

impl IdentifierLiteral {
    ast_leaf_default_new! {IdentifierLiteral,IDENTIFIER }
}

ast_leaf_default_impl! {IdentifierLiteral,TokenIdentifier}

ast_leaf_factory_default_impl! {IdentifierLiteralFactory,IdentifierLiteral}

ast_leaf_default_eval_impl!{ IdentifierLiteral,IDENTIFIER, NAME }