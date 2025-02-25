use crate::ast::ast_list::AstList;
use crate::{ast_list_default_impl, ast_list_default_new, ast_list_factory_default_impl};
use crate::eval::environment::EnvWrapper;
use crate::eval::eval::{EvalRes, Evaluate};

#[derive(Debug, Clone, PartialEq)]
struct Arguments {
    children: AstList
}

impl Arguments {
    ast_list_default_new!{ Arguments }
}

ast_list_default_impl!{ Arguments }

ast_list_factory_default_impl!{ ArgumentsFactory,Arguments }

impl Evaluate for Arguments {
    fn do_eval(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        todo!()
    }
}