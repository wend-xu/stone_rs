use std::any::{Any, TypeId};
use crate::ast::ast_list::AstList;
use crate::eval::environment::EnvWrapper;
use crate::eval::eval::{EvalRes, Evaluate};
use crate::{ast_list_default_impl, ast_list_default_new, ast_list_factory_default_impl};
use crate::ast::ast_tree::AstTree;
use crate::ast::leaf::identifier_literal::IdentifierLiteral;

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterList {
    children: AstList
}

impl ParameterList {
    ast_list_default_new!{ ParameterList }

    pub fn param_name(&self, env: &mut EnvWrapper, index:usize) -> Result<String, String> {
        let param_one =
            self.child_req(index,format!("[Arguments][do_eval_postfix] is None in child index {}",index))?;
        if param_one.actual_type_id() != TypeId::of::<IdentifierLiteral>(){
            Err(format!("[Arguments][do_eval_postfix] param not a IdentifierLiteral in child index {}",index))
        }else{
            Ok(param_one.to_any().downcast_ref::<IdentifierLiteral>().unwrap().id_name())
        }

    }
}

ast_list_default_impl!{ ParameterList }

ast_list_factory_default_impl!{ ParameterListFactory,ParameterList }

impl Evaluate for ParameterList {
    fn do_eval(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        Err("[ParameterList][do_eval] unsupported eval".to_string())
    }
}