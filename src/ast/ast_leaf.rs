use crate::ast::ast_tree::AstTree;
use crate::token::{Token, TokenValue};
use crate::{ast_leaf_impl_for, ast_leaf_new_for};
use std::any::TypeId;
use std::slice::Iter;
use crate::util::str_util::wrapper_node_name;

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
        // panic!("un support in node type [AstLeaf]")
        TypeId::of::<AstLeaf>()
    }
}

pub struct NumberLiteral {
    ast_leaf:AstLeaf,
}

impl NumberLiteral {
    ast_leaf_new_for! {NumberLiteral,NUMBER }
}

ast_leaf_impl_for! {NumberLiteral,TokenNumber}

pub struct IdentifierLiteral {
    ast_leaf:AstLeaf,
}

impl IdentifierLiteral {
    ast_leaf_new_for! {IdentifierLiteral,IDENTIFIER }
}

ast_leaf_impl_for! {IdentifierLiteral,TokenIdentifier}

pub struct StringLiteral {
    ast_leaf: AstLeaf,
}

impl StringLiteral {
    ast_leaf_new_for! {StringLiteral,StringVal }
}

ast_leaf_impl_for! {StringLiteral,TokenText}


