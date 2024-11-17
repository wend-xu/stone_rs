use crate::token::Token;

pub trait Lexer {
    fn read(&mut self) -> Option<Box<dyn Token>>;

    fn peek(&mut self, index: usize) -> Option<&Box<dyn Token>>;
}