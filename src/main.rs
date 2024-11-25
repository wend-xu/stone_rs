use std::fmt::Debug;
use crate::lexer::lexer::Lexer;
use crate::token::Token;
use crate::ast::ast_node::*;
use crate::token::token_number::TokenNumber;

mod lexer;
mod token;
mod tests;
mod util;
mod ast;

fn main() {
    println!("Hello, world!");
    let mut vec:Vec<Box<dyn AstTree>> = Vec::new();

    let token_num = TokenNumber::new(1, 1);
    let leaf = AstLeaf::new(token_num);
    vec.push(leaf);

    //
    let token_num_2 = TokenNumber::new(1, 2);
    let literal = NumberLiteral::new(token_num_2);
    vec.push(literal);
    //
    //
    // println!("{:?}", vec);
    //
    // literal.child(0);
    // leaf.child(0);
}