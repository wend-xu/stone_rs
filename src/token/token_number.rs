use crate::token::token::TokenBase;
use crate::token::Token;

pub struct TokenNumber {
    token_base: TokenBase,
    number: i32,
}

impl TokenNumber {
    fn new(line_number: u32, number: i32) -> TokenNumber {
        TokenNumber {
            token_base: TokenBase::new(line_number),
            number,
        }
    }
}

impl Token<i32> for TokenNumber {
    fn value(&self) -> Option<&i32> {
        Some(&self.number)
    }

    fn line_number(&self) -> &u32 {
        &self.token_base
    }
}