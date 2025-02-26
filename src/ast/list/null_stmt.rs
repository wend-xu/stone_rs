use std::any::TypeId;
use crate::ast::ast_list::AstList;
use crate::{ast_list_factory_default_impl, ast_list_default_impl, ast_list_default_new};
use crate::ast::ast_tree::AstTree;
use crate::eval::environment::EnvWrapper;
use crate::eval::eval::{EvalRes, Evaluate};

#[derive(Debug, Clone, PartialEq)]
pub struct NullStmt {
    children: AstList,
}
impl NullStmt {
    ast_list_default_new! { NullStmt }
}

ast_list_default_impl! { NullStmt }

ast_list_factory_default_impl! {NullStmtFactory,NullStmt}

impl Evaluate for NullStmt {
    fn do_eval(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        panic!("do_eval called on NullStmt")
    }
}

pub fn is_null_stmt(tree_node: &Box<dyn AstTree>) -> bool {
    tree_node.actual_type_id() == TypeId::of::<NullStmt>()
}