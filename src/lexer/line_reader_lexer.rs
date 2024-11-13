use std::fmt::Debug;
use crate::lexer::Lexer;
use regex::{Match, Regex};
use crate::token::Token;

pub const MATCH_COMMENT: &str = "//.";
pub const MATCH_IDENTIFIER: &str = r#"[A-Z_a-z][A-Z_a-z0-9]*|==|<=|>=|&&|\|\||[+\-*/%=\\|&,.!?(){}\[\]><]"#;
pub const MATCH_NUMBER: &str = r"[0-9]+";
pub const MATCH_STRING: &str = r#""((?:\\"|\\\\|\\n|[^"\\])*)""#;

pub struct LineReaderLexer {
    match_line_regex: Regex,
    vec: Vec<Box<dyn Token<dyn Debug>>>,
}

pub fn match_line_regex_str() -> String {
    format!("\\s*((?<comment>{})|(?<number>{})|(?<string>{})|(?<identifier>{}))?", MATCH_COMMENT, MATCH_NUMBER, MATCH_STRING, MATCH_IDENTIFIER)
}

/// read_line : 解析一行代码
impl LineReaderLexer {
    pub fn new() -> LineReaderLexer {
        let match_line = match_line_regex_str();
        let match_line_regex: Regex = Regex::new(match_line.as_str()).unwrap();
        LineReaderLexer { match_line_regex, vec: vec![] }
    }

    pub fn read_line(&self, line: &str, line_number: i32) -> bool {
        let matcher = &self.match_line_regex;

        let mut pos = 0;
        let end_pos = line.len();

        let mut has_more = true;
        while has_more && pos < end_pos {
            let option = matcher.find_at(line, pos);
            has_more = match option {
                None => {
                    println!("line number [{line_number}] find None ,end line find ...");
                    false
                }
                Some(match_r) => {
                    let start = match_r.start();
                    let end = match_r.end();
                    let str_val = match_r.as_str();
                    println!("line number [{line_number}] find Some ,str from {start} to {end} value is [{str_val}] ...");
                    if end == 0 { false } else {
                        pos = end;
                        true
                    }
                }
            }
        }

        false
    }

    fn add_token(&mut self, line_number: u32) {}
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