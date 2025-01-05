use std::any::{Any, TypeId};
use crate::ast::ast_list::AstList;
use crate::{ast_list_factory_default_impl, ast_list_default_impl, ast_list_default_new};
use crate::ast::ast_tree::AstTree;
use crate::ast::list::null_stmt::{is_null_stmt, NullStmt};
use crate::eval::environment::EnvWrapper;
use crate::eval::eval::{EvalRes, Evaluate};

pub struct BlockStmt {
    children: AstList,
}

impl BlockStmt {
    ast_list_default_new! {BlockStmt}
}

ast_list_default_impl! {BlockStmt}

ast_list_factory_default_impl! {BlockStmtFactory,BlockStmt}


// 最后一个语句的结果作为返回值
impl Evaluate for BlockStmt {
    fn do_eval(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        let mut res = EvalRes::VOID;
        let mut iter = self.children();
        while let Some(tree_node) = iter.next() {
            if !is_null_stmt(tree_node) {
                let eval = tree_node.eval();
                res = eval.do_eval(env)?;
            }
        }
        Ok(res)
    }
}