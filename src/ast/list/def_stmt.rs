use std::any::TypeId;
use std::f32::consts::E;
use crate::ast::ast_list::AstList;
use crate::{ast_list_default_impl, ast_list_default_new, ast_list_factory_default_impl, or};
use crate::ast::ast_tree::AstTree;
use crate::ast::leaf::identifier_literal::IdentifierLiteral;
use crate::ast::list::block_stmt::BlockStmt;
use crate::ast::list::paramter_list::ParameterList;
use crate::eval::environment::{Env, EnvWrapper};
use crate::eval::eval::{EvalRes, Evaluate};

#[derive(Debug, Clone, PartialEq)]
struct DefStmt {
    children: AstList,
}

impl DefStmt {
    ast_list_default_new! { DefStmt }

    fn def_name(&self) -> Result<String, String> {
        match self.children.child_downcast::<IdentifierLiteral>(0) {
            Ok(identifier) => { Ok(identifier.id_name()) }
            Err(err_msg) => { Err(format!("[DefStmt][def_name] {err_msg}")) }
        }
    }

    fn param_list(&self) -> Result<ParameterList, String> {
        match self.children.child_downcast::<ParameterList>(1) {
            Ok(param_list) => { Ok(param_list) }
            Err(err_msg) => { Err(format!("[DefStmt][param_list] {err_msg}")) }
        }
    }

    fn block_stmt(&self) -> Result<BlockStmt, String> {
        match self.children.child_downcast::<BlockStmt>(2) {
            Ok(block_stmt) => { Ok(block_stmt) }
            Err(err_msg) => { Err(format!("[DefStmt][block_stmt] {err_msg}")) }
        }
    }
}

ast_list_default_impl! { DefStmt }

ast_list_factory_default_impl! { DefStmtFactory,DefStmt }

impl Evaluate for DefStmt {
    fn do_eval(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        let def_name = self.def_name()?;
        let function = EvalRes::FUNCTION(def_name.clone(), self.param_list()?, self.block_stmt()?);
        env.put(def_name.clone(), function)?;
        Ok(EvalRes::StringVal(def_name))
    }
}