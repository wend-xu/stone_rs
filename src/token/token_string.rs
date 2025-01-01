use crate::token::token::TokenLine;
use crate::token::{Token, TokenValue};

#[derive(Debug)]
pub struct TokenString {
    token_base: TokenLine,
    text: TokenValue,
}


impl TokenString {
    pub fn new(line_number: usize, str: &str) -> Box<TokenString> {
        let string_val =
            if str.len() > 1 && str.starts_with("\"") && str.ends_with("\"") {
                str[1..str.len() - 1].into()
            } else { str.into() };

        Box::new(
            TokenString {
                token_base: TokenLine::new(line_number),
                text: TokenValue::StringVal(string_val),
            }
        )
    }
}


impl Token for TokenString {
    fn value(&self) -> &TokenValue {
        &self.text
    }

    fn line_number(&self) -> &usize {
        &self.token_base
    }
}
