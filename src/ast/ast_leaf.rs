use crate::ast::ast_tree::AstTree;
use crate::ast_leaf_factory_default_impl;
use crate::token::Token;
use crate::util::str_util::wrapper_node_name;
use std::any::TypeId;
use std::slice::Iter;

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
}

ast_leaf_factory_default_impl! {DefAstLeafFactory,AstLeaf}
