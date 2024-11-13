use std::fmt::Debug;
use std::ops::Deref;

// #[derive(Debug)]
// pub enum Token {
//     EOF(TokenEOF),
//     EOL(TokenEOL),
//     IDENTIFIER(i32, String),
//     NUMBER(i32, i32),
//     TEXT(i32, String),
// }

#[derive(Debug)]
pub struct TokenBase {
    line_number: u32,
}

impl TokenBase {
    pub fn new(line_number: u32) -> TokenBase {
        TokenBase { line_number }
    }
}

impl Deref for TokenBase {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.line_number
    }
}

pub trait Token<V:Debug> {
    // fn value(&self) -> Option<V>;
    fn value(&self) -> Option<&V>;

    fn line_number(&self) -> &u32;
}






