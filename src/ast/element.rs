use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::ast_tree::AstTree;
use crate::ast::parser::Parser;
use crate::lexer::lexer::Lexer;

pub trait Element {
    fn parse(&self, lexer: &mut dyn Lexer, res: &mut Vec<Box<dyn AstTree>>) -> Result<(), String>;

    fn is_match(&self, lexer: &mut dyn Lexer) -> bool;
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
            Err(format!("OrTree::choose failed, no parser found, token : [{} : {:?} ]",next_token.line_number(),next_token.value() ))
        }
    }

    fn is_match(&self, lexer: &mut dyn Lexer) -> bool {
        self.choose(lexer).is_some()
    }
}
