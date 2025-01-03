use crate::ast::ast_list::AstList;
use crate::{ast_list_default_impl, ast_list_default_new};
use crate::ast::ast_tree::AstTree;
use crate::parser::factory::AstFactory;

pub struct PrimaryExpr {
    children: AstList,
}
impl PrimaryExpr {
    ast_list_default_new! { PrimaryExpr }
}

ast_list_default_impl! { PrimaryExpr }

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
}

