use crate::token::Token;

pub trait Lexer {
    fn read(&self);

    fn peek(&self,index: usize) -> Box<dyn Token>;
}