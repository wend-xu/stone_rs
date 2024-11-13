use crate::token::token::TokenBase;
use crate::token::Token;

pub struct TokenString {
    token_base: TokenBase,
    string: String,
}


impl TokenString {
    fn new(line_number: u32, str: &str) -> TokenString {
        TokenString {
            token_base: TokenBase::new(line_number),
            string: str.to_string(),
        }
    }
}


impl Token<String> for TokenString {
    fn value(&self) -> Option<&String> {
        Some(&self.string)
    }

    fn line_number(&self) -> &u32 {
        &self.token_base
    }
}
