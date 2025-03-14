use std::fmt::{Debug, Formatter};
use std::ops::Deref;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    EOF,
    EOL,
    IDENTIFIER(String),
    NUMBER(isize),
    StringVal(String),
}

impl TokenValue {
    pub fn literal_eol() -> &'static str {
        "\n"
    }

    pub fn to_string(&self) -> String {
        match self {
            TokenValue::EOF => { "EOF".to_string() }
            TokenValue::EOL => { "\n".to_string() }
            TokenValue::IDENTIFIER(id) => { id.clone() }
            TokenValue::NUMBER(num) => { num.to_string() }
            TokenValue::StringVal(str) => { str.clone() }
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            TokenValue::NUMBER(_) => { true }
            _ => { false }
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            TokenValue::StringVal(_) => { true }
            _ => { false }
        }
    }

    pub fn is_identifier(&self) -> bool {
        match self {
            TokenValue::IDENTIFIER(_) => { true }
            _ => { false }
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


#[derive(Debug, Clone, PartialEq)]
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

    fn clone_token(&self) -> Box<dyn Token>;
}

impl Clone for Box<dyn Token> {
    fn clone(&self) -> Self {
        self.clone_token()
    }
}


impl PartialEq for Box<dyn Token> {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value() && self.line_number() == other.line_number()
    }
}


impl Debug for Box<dyn Token> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.deref().value().fmt(f)?;
        self.deref().line_number().fmt(f)
    }
}
