use crate::ast::ast_list::AstList;
use crate::{ast_impl_list_factory, ast_list_impl_for, ast_list_new_for};

pub struct BlockStmt {
    children: AstList,
}

impl BlockStmt {
    ast_list_new_for! {BlockStmt}
}

ast_list_impl_for! {BlockStmt}

ast_impl_list_factory! {BlockStmtFactory,BlockStmt}