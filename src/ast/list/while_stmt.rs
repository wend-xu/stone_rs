use crate::ast::ast_list::AstList;
use crate::{ast_list_factory_default_impl, ast_list_default_impl, ast_list_default_new};

pub struct WhileStmt {
    children: AstList,
}
impl WhileStmt {
    ast_list_default_new! { WhileStmt }
}
ast_list_default_impl! { WhileStmt }

ast_list_factory_default_impl! {WhileStmtFactory,WhileStmt}