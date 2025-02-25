use std::any::Any;
use crate::ast::ast_list::AstList;
use crate::eval::environment::EnvWrapper;
use crate::eval::eval::{EvalRes, Evaluate};
use crate::{ast_list_default_impl, ast_list_default_new, ast_list_factory_default_impl};
use crate::ast::ast_tree::AstTree;

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterList {
    children: AstList
}

impl ParameterList {
    ast_list_default_new!{ ParameterList }
}

ast_list_default_impl!{ ParameterList }

ast_list_factory_default_impl!{ ParameterListFactory,ParameterList }

impl Evaluate for ParameterList {
    fn do_eval(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        todo!()
    }
}