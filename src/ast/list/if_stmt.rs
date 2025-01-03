use crate::ast::ast_list::AstList;
use crate::{ast_list_factory_default_impl, ast_list_default_impl, ast_list_default_new};

pub struct IfStmt {
    children: AstList,
}

impl IfStmt {
    ast_list_default_new! {IfStmt}
}

ast_list_default_impl! {IfStmt}

ast_list_factory_default_impl! {IfStmtFactory,IfStmt}