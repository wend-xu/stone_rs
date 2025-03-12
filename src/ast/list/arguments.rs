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
                if param_list.num_children() != self.children.num_children() {
                    return Err(format!("[Arguments][do_eval_postfix] incorrect arg num {} , need {} in function's param list",
                                       self.children.num_children(), param_list.num_children()));
                }

                // 捕捉执行环境后，将环境的引入转入被内部
                let mut nest_env = MapNestedEnv::new();
                for (index,arg) in self.children().enumerate() {
                    let arg_val = arg.eval().do_eval(env)?;
                    let param_one =
                        param_list.child_req(index,format!("[Arguments][do_eval_postfix] is None in child index {}",index))?;
                    if param_one.actual_type_id() != TypeId::of::<IdentifierLiteral>(){
                        return Err(format!("[Arguments][do_eval_postfix] param not a IdentifierLiteral in child index {}",index));
                    }
                    let param_name = param_one.to_any().downcast_ref::<IdentifierLiteral>().unwrap().id_name();
                    nest_env.put(param_name, arg_val)?;
                }
                nest_env.set_outer(env);
                let mut nest_env = nest_env.wrapper();
                Ok(func_block.do_eval(&mut nest_env)?)
            }
            _ => {
                Err("[Arguments][do_eval_postfix] not a function, can not eval , current function type is EvalRes::FUNCTION".to_string())
            }
        }
    }
}