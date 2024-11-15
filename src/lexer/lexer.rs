use crate::token::Token;

pub trait Lexer {
    fn read(&self) -> Option<Box<dyn Token>>;

    fn peek(&self, index: usize) -> Option<&Box<dyn Token>>;
}