use crate::ast::ast_list::AstList;
use crate::ast::ast_tree::AstTree;
use crate::token::Token;

pub trait AstFactory {
    fn make(&self, res: Vec<Box<dyn AstTree>>) -> Box<dyn AstTree>;
}


pub trait AstLeafFactory {
    fn make(&self, res: Box<dyn Token>) -> Box<dyn AstTree>;
}
pub struct AstListFactory {}
impl AstListFactory {
    pub fn new() -> Box<Self> {
        Box::new(AstListFactory {})
    }
}
impl AstFactory for AstListFactory {
    fn make(&self, res: Vec<Box<dyn AstTree>>) -> Box<dyn AstTree> {
        let mut res = res;
        if res.len() == 1 {
            res.remove(0)
        } else { Box::new(AstList::new_def(res)) }
    }
}



