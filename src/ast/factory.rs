use crate::ast::ast_list::{AstList, BinaryExpr};
use crate::ast::ast_tree::AstTree;

pub trait AstFactory {
    fn make(&self, res: Vec<Box<dyn AstTree>>) -> Box<dyn AstTree>;
}

pub struct BinaryExprFactory {}

impl BinaryExprFactory {
    pub fn new() -> Self {
        BinaryExprFactory {}
    }
}

impl AstFactory for BinaryExprFactory {
    fn make(&self, res: Vec<Box<dyn AstTree>>) -> Box<dyn AstTree> {
        Box::new(BinaryExpr::new(res))
    }
}



pub struct AstListFactory {}

impl AstListFactory {
    pub fn new() -> Box<Self>{
        Box::new(AstListFactory {})
    }
}


impl AstFactory for AstListFactory {
    fn make(&self, res: Vec<Box<dyn AstTree>>) -> Box<dyn AstTree> {
        Box::new(AstList::new(res))
    }
}