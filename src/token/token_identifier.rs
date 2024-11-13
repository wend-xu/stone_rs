use crate::token::token::TokenBase;
use crate::token::Token;

///标识符(identifier)指的是变量名、函数名或类名等名称。此外，+或-等运算符及括号等标
/// 点符号也属于标识符。标点符号与保留字有时也会被归为另一种类型的单词，不过Stone语言在
/// 实现时没有对它们加以区分，都作为标识符处理。
#[derive(Debug)]
pub struct TokenIdentifier {
    token_base: TokenBase,
    identifier: String,
}

impl TokenIdentifier {
    pub fn new(line_number: u32, identifier: String) -> TokenIdentifier {
        TokenIdentifier {
            token_base: TokenBase::new(line_number),
            identifier,
        }
    }
}

impl Token<String> for TokenIdentifier {
    fn value(&self) -> Option<&String> {
        Some(&self.identifier)
    }


    fn line_number(&self) -> &u32 {
        &self.token_base
    }
}
