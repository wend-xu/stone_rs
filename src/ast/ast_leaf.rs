use std::ops::Deref;
use crate::{ast_leaf_for, ast_leaf_new_for};
use crate::ast::ast_tree::AstTree;
use crate::token::Token;
use crate::token::token_identifier::TokenIdentifier;
use crate::token::token_number::TokenNumber;
use crate::token::token_string::TokenText;

pub struct AstLeaf<T: Token> {
    token: Box<T>,
}

impl<T: Token> AstLeaf<T> {
    pub fn new(token: Box<T>) -> Box<AstLeaf<T>> {
        Box::new(AstLeaf { token })
    }
}

impl<T: Token> AstTree for AstLeaf<T> {
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
    ast_leaf: Box<AstLeaf<TokenNumber>>,
}

impl NumberLiteral {
    ast_leaf_new_for! {NumberLiteral,TokenNumber}
}

ast_leaf_for! {NumberLiteral,TokenNumber}

pub struct IdentifierLiteral {
    ast_leaf: Box<AstLeaf<TokenIdentifier>>,
}

impl IdentifierLiteral {
    ast_leaf_new_for! {IdentifierLiteral,TokenIdentifier}
}

ast_leaf_for! {IdentifierLiteral,TokenIdentifier}

struct StringLiteral {
    ast_leaf: Box<AstLeaf<TokenText>>,
}

impl StringLiteral {
    ast_leaf_new_for! {StringLiteral,TokenText}
}

ast_leaf_for! {StringLiteral,TokenText}


