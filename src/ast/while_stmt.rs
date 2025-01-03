use crate::ast::ast_list::AstList;
use crate::{ast_impl_list_factory, ast_list_impl_for, ast_list_new_for};

pub struct WhileStmt {
    children: AstList,
}
impl WhileStmt {
    ast_list_new_for! { WhileStmt }
}
ast_list_impl_for! { WhileStmt }

ast_impl_list_factory! {WhileStmtFactory,WhileStmt}