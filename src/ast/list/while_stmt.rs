use crate::ast::ast_list::AstList;
use crate::{ast_list_factory_default_impl, ast_list_default_impl, ast_list_default_new};
use crate::eval::environment::EnvWrapper;
use crate::eval::eval::{EvalRes, Evaluate};

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStmt {
    children: AstList,
}
impl WhileStmt {
    ast_list_default_new! { WhileStmt }

    fn condition(&self) -> Result<Box<&dyn Evaluate>, String> {
        self.children.child_as_eval(0, "[WhileStmt] condition is None".to_string())
    }

    fn body(&self) -> Result<Box<&dyn Evaluate>, String> {
        self.children.child_as_eval(1, "[WhileStmt] body is None".to_string())
    }
}
ast_list_default_impl! { WhileStmt }

ast_list_factory_default_impl! {WhileStmtFactory,WhileStmt}

impl Evaluate for WhileStmt {
    fn do_eval(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        loop {
            let condition_res = self.condition()?.do_eval(env)?;
            if condition_res == EvalRes::BOOLEAN(false) {
                return Ok(EvalRes::VOID);
            } else {
                self.body()?.do_eval(env)?;
            }
        }
    }
}