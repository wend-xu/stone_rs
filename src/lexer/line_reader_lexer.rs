use crate::lexer::lexer::Lexer;
use crate::token::token_end::TokenEOL;
use crate::token::token_identifier::TokenIdentifier;
use crate::token::token_number::TokenNumber;
use crate::token::token_string::TokenString;
use crate::token::Token;
use crate::util::regex_util::get_from_captures;
use regex::{Captures, Regex};
use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};

pub const MATCH_COMMENT: &str = "//.";
pub const MATCH_IDENTIFIER: &str = r#"[A-Z_a-z][A-Z_a-z0-9]*|==|<=|>=|&&|\|\||[+\-*/%=\\|&,.!?(){}\[\]><:]"#;
pub const MATCH_NUMBER: &str = r"[0-9]+";
pub const MATCH_STRING: &str = r#""((?:\\"|\\\\|\\n|[^"\\])*)""#;

// todo 融合进  MatchNames
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
            /// 未知的字面量类型不应该存在，直接 panic
            _ => panic!("[MatchNames] not exist type literal is {literal}"),
        }
    }

    fn literal(&self) -> &str {
        match self {
            MatchNames::Comment => { "comment" }
            MatchNames::Number => { "number" }
            MatchNames::String => { "string" }
            MatchNames::Identifier => { "identifier" }
        }
    }
}

pub struct LineReaderLexer {
    match_line_regex: Regex,
    code: Vec<String>,
    current_line: usize,
    token_queue: VecDeque<Box<dyn Token>>,
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
            current_line: 0,
            token_queue: VecDeque::new(),
        }
    }

    pub fn read_line(&mut self) -> &mut LineReaderLexer {
        let current_line = self.current_line;
        let code_line = self.code.len();
        if current_line >= code_line {
            return self;
        }

        let matcher = &self.match_line_regex;
        let line_code = self.code[current_line].trim();

        let mut pos = 0;
        let end_pos = line_code.len();
        let mut has_more = true;
        while has_more && pos < end_pos {
            let option = matcher.captures_at(line_code, pos);
            has_more = match option {
                None => {
                    // println!("line number [{line_number}] find None ,end line find ...");
                    false
                }
                Some(cap) => {
                    let (next_pos, token_op) = Self::gen_token(&current_line, cap);
                    pos = next_pos;
                    Self::enqueue_token_op(&mut self.token_queue, &next_pos, token_op)
                }
            }
        }
        Self::enqueue_token(&mut self.token_queue, TokenEOL::new(current_line.clone()));
        self.current_line = current_line + 1;
        self
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
                    MatchNames::String => { Some(TokenString::new(line_number_clone, token_literal)) }
                    MatchNames::Identifier => { Some(TokenIdentifier::new(line_number_clone, token_literal)) }
                }
            } else { None };
        (next_index, token_some)
    }

    /// matcher 的不可变借用在 while 循环内持续有效，
    /// 故此时不能对整个 self 创建可变引用
    /// 只能对 self 的其他属性创建可变引用
    fn enqueue_token_op(vec: &mut VecDeque<Box<dyn Token>>, next_pos: &usize, token_op: Option<Box<dyn Token>>) -> bool {
        match token_op {
            None => { false }
            Some(token_some) => {
                vec.push_back(token_some);
                *next_pos != 0
            }
        }
    }

    fn enqueue_token(vec: &mut VecDeque<Box<dyn Token>>, token: Box<dyn Token>) {
        vec.push_back(token);
    }
}

impl Display for LineReaderLexer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[LineReaderLexer] \n")?;
        write!(f, "\tmatch regex : {} \n", self.match_line_regex)?;
        let mut line_num: usize = 0;
        let vec_bor = &self.token_queue;
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
    fn read(&mut self) -> Option<Box<dyn Token>> {
        let option = self.token_queue.pop_front();
        if option.is_none() { self.read_line().token_queue.pop_front() } else { option }
    }

    fn peek(&mut self, index: usize) -> Option<&Box<dyn Token>> {
        let mut try_read_line = false;
        {
            let option = self.token_queue.get(index);
            try_read_line = option.is_none();
        }
        if try_read_line {
            self.read_line().token_queue.get(index)
        } else { self.token_queue.get(index) }
    }
}