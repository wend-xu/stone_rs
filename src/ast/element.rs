use crate::ast::ast_tree::AstTree;
use crate::lexer::lexer::Lexer;

pub trait Element {
    fn parse(&self,lexer:dyn Lexer,res:Vec<Box<dyn AstTree>>);

    fn match_fn(&self,lexer:dyn Lexer) -> bool;
}