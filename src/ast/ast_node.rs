use std::ops::Deref;
use crate::default_ast_leaf;
use crate::token::Token;
use crate::token::token_number::TokenNumber;

pub trait AstTree {
    fn child(&self, index: usize) -> Option<Box<dyn AstTree>>;

    fn num_children(&self) -> usize;

    fn children(&self) -> Option<Box<dyn AstTree>>;

    fn location(&self) -> String;
}

pub struct AstLeaf {
    token:Box<dyn Token>,
}

impl AstLeaf {
    pub fn new(token: Box<dyn Token>) -> Box<AstLeaf> {
        Box::new(AstLeaf { token })
    }
}

impl AstTree for AstLeaf {
    fn child(&self, index: usize) -> Option<Box<dyn AstTree>> {
        None
    }

    fn num_children(&self) -> usize {
        0
    }

    fn children(&self) -> Option<Box<dyn AstTree>> {
        None
    }

    fn location(&self) -> String {
        format!("<location:{}>", self.token.line_number())
    }
}

pub struct NumberLiteral {
    ast_leaf: Box<AstLeaf>,
}

impl NumberLiteral {
    pub fn new(token: Box<TokenNumber>) -> Box<NumberLiteral> {
        Box::new(NumberLiteral { ast_leaf: AstLeaf::new(token) })
    }
}

default_ast_leaf!{NumberLiteral}