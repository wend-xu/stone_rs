use crate::ast::ast_list::BinaryExpr;
use std::any::TypeId;
use std::slice::Iter;

pub trait AstTree {
    fn child(&self, index: usize) -> Option<&Box<dyn AstTree>>;

    fn num_children(&self) -> usize;

    fn children(&self) -> Iter<Box<dyn AstTree>>;

    fn location(&self) -> String;

    fn actual_type_id(&self) -> TypeId;
}

pub trait AstFactory{
    type Item;

    fn make(&self,res: Vec<Box<dyn AstTree>>) -> Self::Item;
}

pub struct BinaryExprFactory{}

impl AstFactory for BinaryExprFactory {
    type Item = BinaryExpr;
    fn make(&self,res: Vec<Box<dyn AstTree>>) -> BinaryExpr {
        BinaryExpr::new(res)
    }
}

