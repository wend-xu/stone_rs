use crate::ast::ast_list::AstList;
use crate::{ast_list_default_impl, ast_list_default_new, ast_list_factory_default_impl};
use crate::ast::list::block_stmt::BlockStmt;
use crate::ast::list::paramter_list::ParameterList;
use crate::eval::environment::EnvWrapper;
use crate::eval::eval::{EvalRes, Evaluate};

#[derive(Debug, Clone, PartialEq)]
struct FunStmt {
    children: AstList,
}

impl FunStmt {
    ast_list_default_new! { FunStmt }

    fn param_list(&self) -> Result<ParameterList, String> {
        match self.children.child_downcast::<ParameterList>(0) {
            Ok(param_list) => { Ok(param_list) }
            Err(err_msg) => { Err(format!("[FunStmt][param_list] {err_msg}")) }
        }
    }

    fn block_stmt(&self) -> Result<BlockStmt, String> {
        match self.children.child_downcast::<BlockStmt>(1) {
            Ok(block_stmt) => { Ok(block_stmt) }
            Err(err_msg) => { Err(format!("[FunStmt][block_stmt] {err_msg}")) }
        }
    }
}

ast_list_default_impl! { FunStmt }

ast_list_factory_default_impl! { FunStmtFactory,FunStmt }

impl Evaluate for FunStmt {
    fn do_eval(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        let function = EvalRes::FUNCTION(None, self.param_list()?, self.block_stmt()?);
        Ok(function)
    }
}