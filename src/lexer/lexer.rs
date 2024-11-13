use std::fmt::Debug;
use crate::token::Token;

pub trait Lexer {
    fn read(&self);

    // fn peek(&self,index: usize) -> Option<dyn Token<dyn Debug>>;
}