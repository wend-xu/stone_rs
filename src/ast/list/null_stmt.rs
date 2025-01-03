use crate::ast::ast_list::AstList;
use crate::{ast_impl_list_factory, ast_list_impl_for, ast_list_new_for};

pub struct NullStmt {
    children: AstList,
}
impl NullStmt {
    ast_list_new_for! { NullStmt }
}
ast_list_impl_for! { NullStmt }

ast_impl_list_factory! {NullStmtFactory,NullStmt}

