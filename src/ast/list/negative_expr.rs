use crate::ast::ast_list::AstList;
use crate::{ast_list_factory_default_impl, ast_list_default_impl, ast_list_default_new};

pub struct NegativeExpr {
    children: AstList,
}
impl NegativeExpr {
    ast_list_default_new! { NegativeExpr }
}
ast_list_default_impl! { NegativeExpr }

ast_list_factory_default_impl! {NegativeExprFactory,NegativeExpr}