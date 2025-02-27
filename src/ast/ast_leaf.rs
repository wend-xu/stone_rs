use crate::ast::ast_tree::AstTree;
use crate::ast_leaf_factory_default_impl;
use crate::eval::eval::Evaluate;
use crate::token::Token;
use crate::util::str_util::wrapper_node_name;
use std::any::{Any, TypeId};
use std::slice::Iter;

#[derive(Debug, Clone)]
pub struct AstLeaf {
    token: Box<dyn Token>,
}

impl AstLeaf {
    pub fn new(token: Box<dyn Token>) -> Box<Self> {
        Box::new(AstLeaf { token })
    }

    pub fn new_un_ref(token: Box<dyn Token>) -> Self {
        AstLeaf { token }
    }

    pub fn token(&self) -> &Box<dyn Token> { &self.token }
}

impl AstTree for AstLeaf {
    fn child(&self, index: usize) -> Option<&Box<dyn AstTree>> {
        None
    }

    fn num_children(&self) -> usize {
        0
    }

    fn children(&self) -> Iter<Box<dyn AstTree>> {
        let empty_slice: &[Box<dyn AstTree>] = &[];
        empty_slice.iter()
    }

    fn location(&self) -> String {
        let location = format!("<value :{:?}>", self.token.value());
        wrapper_node_name(location)
    }

    fn actual_type_id(&self) -> TypeId {
        TypeId::of::<AstLeaf>()
    }

    fn eval(&self) -> Box<&dyn Evaluate> {
        panic!("[AstLeaf][eval] unsupported eval type");
    }

    fn to_any(&self) -> &dyn Any {
        self
    }

    fn clone_tree(&self) -> Box<dyn AstTree> {
       Box::new(self.clone())
    }

    fn eq_tree(&self, other: &dyn AstTree) -> bool {
        if self.actual_type_id() == TypeId::of::<AstLeaf>() {
            self == other.to_any().downcast_ref::<AstLeaf>().unwrap()
        }else{ false }
    }
}

ast_leaf_factory_default_impl! {DefAstLeafFactory,AstLeaf}

impl PartialEq for AstLeaf {
    fn eq(&self, other: &Self) -> bool {
        self.token.eq(&other.token)
    }
}

