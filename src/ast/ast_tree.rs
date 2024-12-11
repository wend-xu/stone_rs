use std::any::TypeId;
use std::slice::Iter;
use crate::ast::ast_leaf::AstLeaf;
use crate::ast::ast_list::BinaryExpr;

pub trait AstTree {
    fn child(&self, index: usize) -> Option<&Box<dyn AstTree>>;

    fn num_children(&self) -> usize;

    fn children(&self) -> Iter<Box<dyn AstTree>>;

    fn location(&self) -> String;

    fn actual_type_id(&self) -> TypeId;
}

pub trait AstFactory<T:AstTree>{

    fn make(&self,res: Vec<Box<dyn AstTree>>) -> T;
}

impl AstFactory<BinaryExpr> for BinaryExpr {
    fn make(&self,res: Vec<Box<dyn AstTree>>) -> BinaryExpr {
        BinaryExpr::new(res)
    }
}