use std::any::{Any, TypeId};
use std::slice::Iter;
use crate::eval::eval::Evaluate;

pub trait AstTree {
    fn child(&self, index: usize) -> Option<&Box<dyn AstTree>>;

    fn num_children(&self) -> usize;

    fn children(&self) -> Iter<Box<dyn AstTree>>;

    fn location(&self) -> String;

    fn actual_type_id(&self) -> TypeId;

    fn eval(&self) -> Box<&dyn Evaluate>;

    fn to_any (&self) -> &dyn Any;

    fn clone_tree(&self) -> Box<dyn AstTree>;

    fn eq_tree(&self, other:&dyn AstTree) -> bool;

    // fn eq_tree(&self, other:&dyn AstTree) -> bool{
    //     false
    // }
}




