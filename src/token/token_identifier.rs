use crate::token::token::TokenLine;
use crate::token::{Token, TokenValue};

///标识符(identifier)指的是变量名、函数名或类名等名称。此外，+或-等运算符及括号等标
/// 点符号也属于标识符。标点符号与保留字有时也会被归为另一种类型的单词，不过Stone语言在
/// 实现时没有对它们加以区分，都作为标识符处理。
#[derive(Debug)]
pub struct TokenIdentifier {
    token_line: TokenLine,
    identifier: TokenValue,
}

impl TokenIdentifier {
    pub fn new(line_number: usize, identifier: &str) -> Box<TokenIdentifier> {
        Box::new(
            TokenIdentifier {
                token_line: TokenLine::new(line_number),
                identifier: TokenValue::IDENTIFIER(identifier.to_string()),
            }
        )
    }
}

impl Token for TokenIdentifier {
    fn value(&self) -> &TokenValue {
        &self.identifier
    }


    fn line_number(&self) -> &usize {
        &self.token_line
    }
}
