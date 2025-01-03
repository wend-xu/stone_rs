use crate::ast::ast_list::AstList;
use crate::{ast_list_factory_default_impl, ast_list_default_impl, ast_list_default_new};

pub struct NullStmt {
    children: AstList,
}
impl NullStmt {
    ast_list_default_new! { NullStmt }
}
ast_list_default_impl! { NullStmt }

ast_list_factory_default_impl! {NullStmtFactory,NullStmt}

