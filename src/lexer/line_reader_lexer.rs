use crate::token::token_identifier::TokenIdentifier;
use crate::token::token_number::TokenNumber;
use crate::token::token_string::TokenString;
use crate::token::Token;
use crate::util::regex_util::get_from_captures;
use regex::{Captures, Regex};

pub const MATCH_COMMENT: &str = "//.";
pub const MATCH_IDENTIFIER: &str = r#"[A-Z_a-z][A-Z_a-z0-9]*|==|<=|>=|&&|\|\||[+\-*/%=\\|&,.!?(){}\[\]><]"#;
pub const MATCH_NUMBER: &str = r"[0-9]+";
pub const MATCH_STRING: &str = r#""((?:\\"|\\\\|\\n|[^"\\])*)""#;


pub const MATCH_NAMES: [&str; 4] = ["comment", "number", "string", "identifier"];

pub enum MatchNames {
    Comment,
    Number,
    String,
    Identifier,
}

impl MatchNames {
    fn iternal_all() -> &'static [&'static str; 4] {
        &MATCH_NAMES
    }

    fn of_iternal(iternal: &str) -> MatchNames {
        match iternal {
            "comment" => MatchNames::Comment,
            "number" => MatchNames::Number,
            "string" => MatchNames::String,
            "identifier" => MatchNames::Identifier,
            _ => panic!("[MatchNames] not exist type iternal is {iternal}"),
        }
    }

    fn iternal(&self) -> &str {
        match self {
            MatchNames::Comment => { MATCH_NAMES[0] }
            MatchNames::Number => { MATCH_NAMES[1] }
            MatchNames::String => { MATCH_NAMES[2] }
            MatchNames::Identifier => { MATCH_NAMES[3] }
        }
    }
}

pub struct LineReaderLexer {
    match_line_regex: Regex,
    vec: Vec<Box<dyn Token>>,
}

pub fn match_line_regex_str() -> String {
    format!("\\s*((?<{}>{})|(?<{}>{})|(?<{}>{})|(?<{}>{}))?",
            MATCH_NAMES[0], MATCH_COMMENT, MATCH_NAMES[1], MATCH_NUMBER,
            MATCH_NAMES[2], MATCH_STRING, MATCH_NAMES[3], MATCH_IDENTIFIER)
}

/// read_line : 解析一行代码
impl LineReaderLexer {
    pub fn new() -> LineReaderLexer {
        let match_line = match_line_regex_str();
        let match_line_regex: Regex = Regex::new(match_line.as_str()).unwrap();
        LineReaderLexer { match_line_regex, vec: vec![] }
    }

    pub fn read_line(&mut self, line: &str, line_number: usize) -> bool {
        let matcher = &self.match_line_regex;

        let mut pos = 0;
        let end_pos = line.len() as usize;

        let mut has_more = true;
        while has_more && pos < end_pos {
            let option = matcher.captures_at(line, pos);
            has_more = match option {
                None => {
                    println!("line number [{line_number}] find None ,end line find ...");
                    false
                }
                Some(cap) => {
                    let (next_pos, token_op) = Self::gen_token(line_number, cap);
                    if let Some(token_some) = token_op {
                        println!("line number [{line_number}] found token {:?} had parse ... ", token_some.value());
                        self.vec.push(token_some);
                    }
                    if next_pos == 0 { false } else {
                        pos = next_pos;
                        true
                    }
                }
            }
        }
        false
    }

    fn gen_token(line_number: usize, cap: Captures) -> (usize, Option<Box<dyn Token>>) {
        let mut next_index = 0;
        let token_some: Option<Box<dyn Token>> =
            if let Some((name, token_iternal, _, end)) = get_from_captures(&cap, MatchNames::iternal_all()) {
                next_index = end;
                // 每次都克隆一个 行号， 将所有权转移到 Token
                let line_number_clone = line_number.clone();
                match MatchNames::of_iternal(&name) {
                    MatchNames::Comment => { None }
                    MatchNames::Number => { Some(TokenNumber::new_iternal(line_number_clone, token_iternal)) }
                    MatchNames::String => { Some(TokenString::new(line_number_clone, token_iternal)) }
                    MatchNames::Identifier => { Some(TokenIdentifier::new(line_number_clone, token_iternal)) }
                }
            } else { None };
        (next_index, token_some)
    }
}

// impl Lexer for LineReaderLexer {
//     fn read(&self) {
//         todo!()
//     }
//
//     fn peek(&self, index: usize) -> Option<dyn Token<dyn Debug>> {
//         todo!()
//     }
// }