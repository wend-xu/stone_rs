use std::any::Any;
use std::ops::Deref;
use crate::ast::ast_list::AstList;
use crate::ast::ast_tree::AstTree;
use crate::ast::parser::Parser;
use crate::lexer::lexer::Lexer;
use crate::token::Token;
use crate::util::type_util::struct_is_type;
use std::rc::Rc;
use crate::token::token_identifier::TokenIdentifier;

pub trait Element {
    fn parse(&self, lexer: &mut dyn Lexer, res: &mut Vec<Box<dyn AstTree>>) -> Result<(), String>;

    fn is_match(&self, lexer: &mut dyn Lexer) -> bool { false }
}


struct Tree {
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
struct OrTree {
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


struct Repeat {
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
            if !struct_is_type::<AstList>(&parse_res) || parse_res.num_children() > 0 {
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

struct IdToken;

impl Element for IdToken {
    fn parse(&self, lexer: &mut dyn Lexer, res: &mut Vec<Box<dyn AstTree>>) -> Result<(), String> {
        Ok(())
    }

    fn is_match(&self, lexer: &mut dyn Lexer) -> bool {
        // match lexer.peek(0) {
        //     None => { false }
        //     Some(box_token) => {
        //         struct_is_type::<TokenIdentifier>(box_token.deref())
        //     }
        // }
        false
    }
}