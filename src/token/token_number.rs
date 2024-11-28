use crate::token::token::TokenLine;
use crate::token::{Token, TokenValue};

#[derive(Debug)]
pub struct TokenNumber {
    token_line: TokenLine,
    number: TokenValue,
}

impl TokenNumber {
    pub fn new(line_number: usize, number: isize) -> Box<TokenNumber> {
        Box::new(
            TokenNumber {
                token_line: TokenLine::new(line_number),
                number: TokenValue::NUMBER(number),
            }
        )
    }

    pub fn new_literal(line_number: usize, number_literal: &str) -> Box<TokenNumber> {
        /// 这是一个转换数字字面量错误的情况，
        /// 这种情况说明分词出了问题，故属于分词器实现的错误，直接 panic 掉
        let number = match number_literal.parse::<isize>() {
            Ok(value) => { value }
            Err(_) => {
                panic!("[TokenNumber][new_literal] number_literal:{number_literal} can't parse to number,\
            must number literal when token init,this issue indicates a segmentation error  ")
            }
        };
        Self::new(line_number, number)
    }
}

impl Token for TokenNumber {
    fn value(&self) -> &TokenValue {
        &self.number
    }

    fn line_number(&self) -> &usize {
        &self.token_line
    }
}