use std::any::TypeId;
use std::slice::Iter;

pub trait AstTree {
    fn child(&self, index: usize) -> Option<&Box<dyn AstTree>>;

    fn num_children(&self) -> usize;

    fn children(&self) -> Iter<Box<dyn AstTree>>;

    fn location(&self) -> String;

    fn actual_type_id(&self) -> TypeId;
}