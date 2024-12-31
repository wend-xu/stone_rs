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

    pub fn is_number(&self) -> bool {
        match self {
            TokenValue::NUMBER(_) => {true}
            _ => {false}
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            TokenValue::StringVal(_) => {true}
            _ => {false}
        }
    }

    pub fn is_identifier(&self) -> bool {
        match self {
            TokenValue::IDENTIFIER(_) => {true}
            _ => {false}
        }
    }
}

impl PartialEq<str> for TokenValue {
    fn eq(&self, other: &str) -> bool {
        match self {
            TokenValue::EOF => { false }
            TokenValue::EOL => { TokenValue::literal_eol() == other }
            TokenValue::IDENTIFIER(id) => { id.as_str() == other }
            TokenValue::NUMBER(_) => { false }
            TokenValue::StringVal(str) => { str.as_str() == other }
        }
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






