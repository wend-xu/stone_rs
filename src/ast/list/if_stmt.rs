use crate::ast::ast_list::AstList;
use crate::{ast_list_factory_default_impl, ast_list_default_impl, ast_list_default_new};
use crate::ast::ast_tree::AstTree;
use crate::eval::environment::EnvWrapper;
use crate::eval::eval::{EvalRes, Evaluate};

pub struct IfStmt {
    children: AstList,
}

impl IfStmt {
    ast_list_default_new! {IfStmt}

    fn condition(&self) -> Result<Box<&dyn Evaluate>, String> {
        self.children.child_as_eval(0, "[IfStmt] condition is None".to_string())
    }

    fn then_block(&self) -> Result<Box<&dyn Evaluate>, String> {
        self.children.child_as_eval(1, "[IfStmt] then block is None".to_string())
    }

    fn else_block(&self) -> Option<&Box<dyn AstTree>> {
        self.child(2)
    }
}

ast_list_default_impl! {IfStmt}

ast_list_factory_default_impl! {IfStmtFactory,IfStmt}

impl Evaluate for IfStmt {
    fn do_eval(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        let res = self.condition()?.do_eval(env)?;
        let block_res =
            if res == EvalRes::BOOLEAN(true) || res == EvalRes::NUMBER(1) {
                self.then_block()?.do_eval(env)?
            } else if let Some(else_block) = self.else_block() {
                else_block.eval().do_eval(env)?
            } else { EvalRes::VOID };
        Ok(block_res)
    }
}