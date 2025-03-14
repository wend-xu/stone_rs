use std::any::TypeId;
use std::cell::RefCell;
use std::f32::consts::E;
use std::rc::Rc;
use crate::ast::ast_list::AstList;
use crate::{ast_list_default_impl, ast_list_default_new, ast_list_factory_default_impl};
use crate::ast::ast_tree::AstTree;
use crate::ast::leaf::identifier_literal::IdentifierLiteral;
use crate::eval::environment::{Env, EnvWrapper, MapNestedEnv};
use crate::eval::eval::{EvalRes, Evaluate};

#[derive(Debug, Clone, PartialEq)]
pub struct Arguments {
    children: AstList,
}

impl Arguments {
    ast_list_default_new! { Arguments }
}

ast_list_default_impl! { Arguments }

ast_list_factory_default_impl! { ArgumentsFactory,Arguments }

impl Evaluate for Arguments {
    fn do_eval(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        panic!("[Arguments][do_eval] is a postfix , just support do_eval_postfix");
    }

    fn do_eval_postfix(&self, env: &mut EnvWrapper, result: EvalRes) -> Result<EvalRes, String> {
        match result {
            EvalRes::FUNCTION(func_name, param_list, func_block) => {
                let mut nest_env = MapNestedEnv::capture(env, self, &param_list)?.wrapper();
                func_block.do_eval(&mut nest_env)
            }
            EvalRes::NativeFun(func_name, func_block_native) => {
                let mut nest_env = MapNestedEnv::capture(env, self, &func_block_native.param_list())?.wrapper();
                func_block_native.do_eval(&mut nest_env)
            }
            _ => {
                Err("[Arguments][do_eval_postfix] not a function, can not eval , current function type is EvalRes::FUNCTION".to_string())
            }
        }
    }
}