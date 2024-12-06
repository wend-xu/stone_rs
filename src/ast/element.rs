use crate::ast::ast_leaf::{AstLeaf, IdentifierLiteral, NumberLiteral, StringLiteral};
use crate::ast::ast_list::AstList;
use crate::ast::ast_tree::AstTree;
use crate::ast::parser::Parser;
use crate::ast_impl_element_terminal;
use crate::lexer::lexer::Lexer;
use crate::token::{Token, TokenValue};
use std::any::TypeId;
use std::collections::HashMap;
use std::iter::Map;
use std::rc::Rc;

pub trait Element {
    fn parse(&self, lexer: &mut dyn Lexer, res: &mut Vec<Box<dyn AstTree>>) -> Result<(), String>;

    fn is_match(&self, lexer: &mut dyn Lexer) -> bool { false }
}


pub struct Tree {
    parser: Rc<Parser>,
}

impl Tree {
    fn new(parser: Rc<Parser>) -> Self {
        Tree { parser }
    }
}


impl Element for Tree {
    fn parse(&self, lexer: &mut dyn Lexer, res: &mut Vec<Box<dyn AstTree>>) -> Result<(), String> {
        res.push(self.parser.parse(lexer));
        Ok(())
    }

    fn is_match(&self, lexer: &mut dyn Lexer) -> bool {
        self.parser.is_match(lexer)
    }
}

/// todo 完成后需要确认Rc指针的使用是否造成副作用如内存泄漏
pub struct OrTree {
    parser_vec: Vec<Rc<Parser>>,
}

impl OrTree {
    fn new(parser_vec: Vec<Rc<Parser>>) -> Self {
        OrTree { parser_vec }
    }

    fn choose(&self, lexer: &mut dyn Lexer) -> Option<Rc<Parser>> {
        let mut choose_tree: Option<Rc<Parser>> = None;
        while let Some(parser) = self.parser_vec.iter().next() {
            if parser.is_match(lexer) {
                choose_tree = Some(Rc::clone(parser));
                break;
            }
        }
        choose_tree
    }
}

impl Element for OrTree {
    fn parse(&self, lexer: &mut dyn Lexer, res: &mut Vec<Box<dyn AstTree>>) -> Result<(), String> {
        let choose_tree = self.choose(lexer);
        if let Some(parser) = choose_tree {
            res.push(parser.parse(lexer));
            Ok(())
        } else {
            let next_token = lexer.peek(0).unwrap();
            Err(format!("OrTree::choose failed, no parser found, token : [{} : {:?} ]", next_token.line_number(), next_token.value()))
        }
    }

    fn is_match(&self, lexer: &mut dyn Lexer) -> bool {
        self.choose(lexer).is_some()
    }
}


pub struct Repeat {
    parser: Rc<Parser>,
    only_once: bool,
}

impl Repeat {
    fn new(parser: Rc<Parser>, only_once: bool) -> Self {
        Repeat { parser, only_once }
    }
}

impl Element for Repeat {
    fn parse(&self, lexer: &mut dyn Lexer, res: &mut Vec<Box<dyn AstTree>>) -> Result<(), String> {
        while self.parser.is_match(lexer) {
            let parse_res = self.parser.parse(lexer);
            /// parser 出现AstList则是factory构建ast节点的时候没有指定类型，实际上没有执行的功能
            /// 这种情况确实可以忽略，因为本身就是无法执行的，在ast树上也没意义
            ///
            /// 按照BNF的语法定义，仅在 block中有 repeat ，定义为 {(";" | EOL) [ statement ]} "}" ,
            /// 这中情况块的最后一行，会match，但是后续匹配结果为空，会触发 parse_res.num_children() = 0
            /// 看似  (";" | EOL) 作为开头很别扭，但是支持重复的模式也需要有个可匹配的模式，这样写对实现来说好像是最为简单的
            /// 若是将  (";" | EOL) 作为重复的结尾，一样可以实现匹配，相对的就是匹配完块后不进入while循环的情况
            ///
            /// 故进入while 循环后的判定条件：  不为AstList(不可执行无意义) 子节点是数为0(实际未匹配可执行内容)
            if parse_res.actual_type_id() == TypeId::of::<AstList>() || parse_res.num_children() > 0 {
                res.push(parse_res);
            }
            if self.only_once {
                break;
            }
        }
        Ok(())
    }

    fn is_match(&self, lexer: &mut dyn Lexer) -> bool {
        self.parser.is_match(lexer)
    }
}

pub struct IdToken;
ast_impl_element_terminal!(IdToken,IdentifierLiteral);
// impl Element for IdToken {
//     fn parse(&self, lexer: &mut dyn Lexer, res: &mut Vec<Box<dyn AstTree>>) -> Result<(), String> {
//         let read = lexer.read().unwrap();
//         res.push(NumberLiteral::new(read));
//         Ok(())
//     }
//
//     fn is_match(&self, lexer: &mut dyn Lexer) -> bool {
//         if let Some(box_token) = lexer.peek(0) {
//             NumberLiteral::is_match(box_token)
//         } else { false }
//     }
// }

pub struct StrToken;
ast_impl_element_terminal!(StrToken,StringLiteral);

pub struct NumToken;
ast_impl_element_terminal!(NumToken,NumberLiteral);

#[derive(Debug)]
pub struct Leaf {
    tokens: Vec<TokenValue>,
}

impl Leaf {
    pub fn new(leaf_literal: Vec<&str>) -> Self {
        Leaf {
            tokens: leaf_literal.iter().map(|str| TokenValue::IDENTIFIER(str.to_string())).collect()
        }
    }
}

impl Element for Leaf {
    fn parse(&self, lexer: &mut dyn Lexer, res: &mut Vec<Box<dyn AstTree>>) -> Result<(), String> {
        res.push(AstLeaf::new(lexer.read().unwrap()));
        Ok(())
    }

    fn is_match(&self, lexer: &mut dyn Lexer) -> bool {
        if let Some(token_value) = self.tokens.get(0) {
            self.tokens.contains(token_value)
        } else { false }
    }
}

pub struct Skip {
    leaf: Leaf,
}

impl Skip {
    pub fn new(leaf_literal: Vec<&str>) -> Self {
        Skip {
            leaf: Leaf::new(leaf_literal)
        }
    }
}

impl Element for Skip {
    fn parse(&self, lexer: &mut dyn Lexer, res: &mut Vec<Box<dyn AstTree>>) -> Result<(), String> {
        Ok(())
    }

    fn is_match(&self, lexer: &mut dyn Lexer) -> bool {
        self.leaf.is_match(lexer)
    }
}

struct Precedence {
    value: usize,
    left_assoc: bool,
}

struct Operators {
    operators: HashMap<String, Precedence>,
}

impl Operators {
    fn new() -> Self {
        Operators {
            operators: HashMap::new(),
        }
    }

    fn add(&mut self, name: &str, precedence:usize, left_assoc: bool) -> &Self {
        self.operators.insert(name.to_string(),Precedence{value:precedence,left_assoc});
        self
    }
}

struct Expr{

}