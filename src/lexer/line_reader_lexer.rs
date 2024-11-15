use crate::lexer::lexer::Lexer;
use crate::token::token_end::{TokenEOF, TokenEOL};
use crate::token::token_identifier::TokenIdentifier;
use crate::token::token_number::TokenNumber;
use crate::token::token_string::TokenText;
use crate::token::Token;
use crate::util::regex_util::get_from_captures;
use regex::{Captures, Regex};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};

pub const MATCH_COMMENT: &str = "//.";
pub const MATCH_IDENTIFIER: &str = r#"[A-Z_a-z][A-Z_a-z0-9]*|==|<=|>=|&&|\|\||[+\-*/%=\\|&,.!?(){}\[\]><:]"#;
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
    fn literal_all() -> &'static [&'static str; 4] {
        &MATCH_NAMES
    }

    fn of_literal(literal: &str) -> MatchNames {
        match literal {
            "comment" => MatchNames::Comment,
            "number" => MatchNames::Number,
            "string" => MatchNames::String,
            "identifier" => MatchNames::Identifier,
            _ => panic!("[MatchNames] not exist type literal is {literal}"),
        }
    }

    fn literal(&self) -> &str {
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
    code: Vec<String>,
    current_line: RefCell<usize>,
    vec: RefCell<VecDeque<Box<dyn Token>>>,
}

pub fn match_line_regex_str() -> String {
    format!("\\s*((?<{}>{})|(?<{}>{})|(?<{}>{})|(?<{}>{}))?",
            MATCH_NAMES[0], MATCH_COMMENT, MATCH_NAMES[1], MATCH_NUMBER,
            MATCH_NAMES[2], MATCH_STRING, MATCH_NAMES[3], MATCH_IDENTIFIER)
}

/// read_line : 解析一行代码
impl LineReaderLexer {
    pub fn new(code: String) -> LineReaderLexer {
        let match_line = match_line_regex_str();
        let match_line_regex: Regex = Regex::new(match_line.as_str()).unwrap();
        Self::new_with_regex(code, match_line_regex)
    }

    pub fn new_with_regex(code: String, regex: Regex) -> LineReaderLexer {
        let match_line_regex: Regex = regex;
        LineReaderLexer {
            match_line_regex,
            code: code.lines().map(|line| line.to_string()).collect(),
            current_line: RefCell::new(0),
            vec: RefCell::new(VecDeque::new()),
        }
    }

    pub fn read_line(&self, line: &str, line_number: usize) {
        let matcher = &self.match_line_regex;

        let mut pos = 0;
        let end_pos = line.len();
        // println!("while parse line : [{line_number}], len is [{end_pos}]");
        let mut has_more = true;
        while has_more && pos < end_pos {
            let option = matcher.captures_at(line, pos);
            has_more = match option {
                None => {
                    // println!("line number [{line_number}] find None ,end line find ...");
                    false
                }
                Some(cap) => {
                    let (next_pos, token_op) = Self::gen_token(&line_number, cap);
                    pos = next_pos;
                    self.enqueue_token_op(&next_pos, token_op)
                }
            }
        }
        self.enqueue_token(TokenEOL::new(line_number.clone()));
        self.current_line.replace(line_number);
    }

    fn gen_token(line_number: &usize, cap: Captures) -> (usize, Option<Box<dyn Token>>) {
        let mut next_index = 0;
        let token_some: Option<Box<dyn Token>> =
            if let Some((name, token_literal, _, end)) = get_from_captures(&cap, MatchNames::literal_all()) {
                next_index = end;
                // 每次都克隆一个 行号， 将所有权转移到 Token
                let line_number_clone = line_number.clone();
                match MatchNames::of_literal(&name) {
                    MatchNames::Comment => { None }
                    MatchNames::Number => { Some(TokenNumber::new_literal(line_number_clone, token_literal)) }
                    MatchNames::String => { Some(TokenText::new(line_number_clone, token_literal)) }
                    MatchNames::Identifier => { Some(TokenIdentifier::new(line_number_clone, token_literal)) }
                }
            } else { None };
        (next_index, token_some)
    }

    fn enqueue_token_op(&self, next_pos: &usize, token_op: Option<Box<dyn Token>>) -> bool {
        match token_op {
            None => { false }
            Some(token_some) => {
                &self.vec.borrow_mut().push_back(token_some);
                *next_pos != 0
            }
        }
    }

    fn enqueue_token(&self, token: Box<dyn Token>) {
        &self.vec.borrow_mut().push_back(token);
    }

    // fn read(&self) -> Option<Box<dyn Token>> {
    //     let option = self.vec.borrow_mut().pop_front();
    //     let current_line = *self.current_line.borrow();
    //     if option.is_none() && current_line < self.code.len() {
    //         self.read_line(self.code[current_line].trim(), current_line);
    //         self.current_line.replace(current_line + 1);
    //         self.vec.borrow_mut().pop_front()
    //     } else {
    //         option
    //     }
    // }
    //
    // fn peek(&self, index: usize) -> Option<&Box<dyn Token>> {
    //     if self.vec.borrow().len() > index { self.vec.borrow().get(index) } else { None }
    // }
}

impl Display for LineReaderLexer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[LineReaderLexer] \n")?;
        write!(f, "\tmatch regex : {} \n", self.match_line_regex)?;
        let mut line_num: usize = 0;
        let vec_bor = &self.vec.borrow();
        write!(f, "\tline number : [0]\n")?;
        for token in vec_bor.iter() {
            if line_num != *token.line_number() {
                line_num = *token.line_number();
                write!(f, "\tline number : [{}]\n", line_num)?;
            }
            write!(f, "\t\t[{:?}]\n", token.value())?;
        }
        write!(f, "[LineReaderLexer] \n")
    }
}

impl Lexer for LineReaderLexer {
    fn read(&self) -> Option<Box<dyn Token>> {
        let option = self.vec.borrow_mut().pop_front();
        let current_line = *self.current_line.borrow();
        if option.is_none() && current_line < self.code.len() {
            self.read_line(self.code[current_line].trim(), current_line);
            self.current_line.replace(current_line + 1);
            self.vec.borrow_mut().pop_front()
        } else {
            option
        }
    }

    fn peek(&self, index: usize) -> Option<&Box<dyn Token>> {
        // if self.vec.borrow().len() > index {
        //     if let Some(token) = &self.vec.borrow().get(index) {
        //         Some(&token)
        //     } else {
        //         None
        //     }
        // } else { None }
        
        None
    }
}