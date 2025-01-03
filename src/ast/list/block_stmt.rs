use crate::ast::ast_list::AstList;
use crate::{ast_list_factory_default_impl, ast_list_default_impl, ast_list_default_new};

pub struct BlockStmt {
    children: AstList,
}

impl BlockStmt {
    ast_list_default_new! {BlockStmt}
}

ast_list_default_impl! {BlockStmt}

ast_list_factory_default_impl! {BlockStmtFactory,BlockStmt}