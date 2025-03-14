use ast::ast_leaf::*;
use crate::ast::ast_tree::AstTree;
use crate::lexer::lexer::Lexer;
use crate::token::token_number::TokenNumber;
use crate::token::Token;
use std::fmt::Debug;
use ast::list::binary_expr::BinaryExpr;
use ast::leaf::number_literal::NumberLiteral;

mod lexer;
mod token;
mod tests;
mod util;
mod ast;
mod parser;
mod eval;
mod stone;
mod native;

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

    let expr = BinaryExpr::new(Vec::new());
    println!("{}",expr.location())
}