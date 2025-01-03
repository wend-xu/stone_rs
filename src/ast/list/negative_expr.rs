use crate::ast::ast_list::AstList;
use crate::{ast_impl_list_factory, ast_list_impl_for, ast_list_new_for};

pub struct NegativeExpr {
    children: AstList,
}
impl NegativeExpr {
    ast_list_new_for! { NegativeExpr }
}
ast_list_impl_for! { NegativeExpr }

ast_impl_list_factory! {NegativeExprFactory,NegativeExpr}