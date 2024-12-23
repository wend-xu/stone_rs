use std::fmt::Debug;
use std::ops::Deref;

#[derive(Debug,PartialEq)]
pub enum TokenValue{
    EOF,
    EOL,
    IDENTIFIER(String),
    NUMBER(isize),
    StringVal(String),
}

impl TokenValue {
    pub fn literal_eol() -> &'static str{
        "\n"
    }
}


#[derive(Debug)]
pub struct TokenLine {
    line_number: usize,
}

impl TokenLine {
    pub fn new(line_number: usize) -> TokenLine {
        TokenLine { line_number }
    }
}

impl Deref for TokenLine {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.line_number
    }
}


pub trait Token {
    fn value(&self) -> &TokenValue;

    fn line_number(&self) -> &usize;
}






