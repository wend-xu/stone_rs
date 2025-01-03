use crate::ast::ast_list::AstList;
use crate::{ast_impl_list_factory, ast_list_impl_for, ast_list_new_for};

pub struct IfStmt {
    children: AstList,
}

impl IfStmt {
    ast_list_new_for! {IfStmt}
}

ast_list_impl_for! {IfStmt}

ast_impl_list_factory! {IfStmtFactory,IfStmt}