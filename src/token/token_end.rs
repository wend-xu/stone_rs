use crate::token::token::TokenBase;
use crate::token::Token;

#[derive(Debug)]
pub struct TokenEOL {
    token_base: TokenBase,
    eol: String,
}

impl TokenEOL {
    pub fn new(line_number: u32) -> TokenEOL {
        TokenEOL {
            token_base: TokenBase::new(line_number),
            eol: String::from("\n"),
        }
    }
}

impl Token<String> for TokenEOL {
    fn value(&self) -> Option<&String> {
        Some(&self.eol)
    }


    fn line_number(&self) -> &u32 {
        &self.token_base
    }
}

#[derive(Debug)]
pub struct TokenEOF {
    token_base: TokenBase,
}


impl TokenEOF {
    fn new(line_number: u32) -> TokenEOF { TokenEOF { token_base: TokenBase::new(line_number) } }
}
impl Token<String> for TokenEOF {
    fn value(&self) -> Option<&String> {
        None
    }

    fn line_number(&self) -> &u32 {
        &self.token_base
    }
}