use crate::ast::ast_list::AstList;
use crate::ast::ast_tree::AstTree;
use crate::token::Token;

pub trait AstFactory {
    fn make(&self, res: Vec<Box<dyn AstTree>>) -> Box<dyn AstTree>;

    fn clone(&self) -> Box<dyn AstFactory>;
}


pub trait AstLeafFactory {
    fn make(&self, res: Box<dyn Token>) -> Box<dyn AstTree>;
}

#[derive(Clone,Copy)]
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
        } else {
            Box::new(AstList::new_def(res))
        }
    }

    fn clone(&self) -> Box<dyn AstFactory> {
        Box::new(Clone::clone(self))
    }
}



