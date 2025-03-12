use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::ast_list::AstList;
use crate::{ast_list_default_impl, ast_list_default_new};
use crate::ast::ast_tree::AstTree;
use crate::eval::environment::EnvWrapper;
use crate::eval::eval::{EvalRes, Evaluate};
use crate::parser::factory::AstFactory;

#[derive(Debug, Clone, PartialEq)]
pub struct PrimaryExpr {
    children: AstList,
}
impl PrimaryExpr {
    ast_list_default_new! { PrimaryExpr }

    fn operand(&self) -> Result<&Box<dyn AstTree>, String> {
        self.child_req(0, "[PrimaryExpr][operand] is None".to_string())
    }

    fn postfix(&self, nest: usize) -> Result<&Box<dyn AstTree>, String> {
        let index = self.num_children() - nest - 1;
        self.child_req(index, format!("[PrimaryExpr][postfix] nest:{} is None", nest))
    }

    fn has_postfix(&self, nest: usize) -> bool {
        self.children.num_children() - nest > 1
    }


    // 从最末尾的 postfix 开始递归，所以第一个 postfix 是最先执行的
    fn eval_sub_expr(&self, env: &mut EnvWrapper, nest: usize) -> Result<EvalRes, String> {
        let res = if self.has_postfix(nest) {
            let res = self.eval_sub_expr(env, nest + 1)?;
            self.postfix(nest)?.eval().do_eval_postfix(env, res)?
        } else {
            self.operand()?.eval().do_eval(env)?
        };
        Ok(res)
    }
}

ast_list_default_impl! { PrimaryExpr }


#[derive(Copy, Clone)]
pub struct PrimaryExprFactory {}
impl PrimaryExprFactory {
    pub fn new() -> Box<Self> {
        Box::new(PrimaryExprFactory {})
    }
}
impl AstFactory for PrimaryExprFactory {
    fn make(&self, res: Vec<Box<dyn AstTree>>) -> Box<dyn AstTree> {
        let mut res = res;
        if res.len() == 1 {
            res.remove(0)
        } else {
            Box::new(PrimaryExpr::new(res))
        }
    }

    fn clone(&self) -> Box<dyn AstFactory> {
        Box::new(Clone::clone(self))
    }
}

impl Evaluate for PrimaryExpr {
    fn do_eval(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        self.eval_sub_expr(env, 0)
        // todo!()
    }
}
