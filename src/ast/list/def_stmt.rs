use crate::ast::ast_list::AstList;
use crate::{ast_list_default_impl, ast_list_default_new, ast_list_factory_default_impl, or};
use crate::eval::environment::EnvWrapper;
use crate::eval::eval::{EvalRes, Evaluate};

struct DefStmt{
    children: AstList
}

impl DefStmt {
    ast_list_default_new!{ DefStmt }
}

ast_list_default_impl!{ DefStmt }

ast_list_factory_default_impl!{ DefStmtFactory,DefStmt }

impl Evaluate for DefStmt {
    fn do_eval(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        todo!()
    }
}