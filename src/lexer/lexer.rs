use crate::token::Token;

pub trait Lexer {
    fn read(&self,script:String);

    fn peek(&self,index: usize) -> Box<dyn Token>;
}