use crate::ast::ast_tree::AstTree;
use crate::ast::element::{Element, Expr, IdToken, Leaf, NumToken, Operators, OrTree, Repeat, Skip, StrToken};
use crate::ast::factory::{AstFactory, AstLeafFactory, AstListFactory};
use crate::lexer::lexer::Lexer;
use std::rc::Rc;

// #[derive(Copy, Clone)]
pub struct Parser {
    factory: Box<dyn AstFactory>,
    elements: Vec<Box<dyn Element>>,
}

impl Parser {
    pub fn rule() -> Parser {
        let factory = AstListFactory::new();
        Self::rule_with_factory(factory)
    }

    pub fn rule_with_factory(factory: Box<dyn AstFactory>) -> Parser {
        Parser { factory, elements: vec![] }
    }


    pub fn parse(&self, lexer: &mut dyn Lexer) -> Result<Box<dyn AstTree>, String> {
        let mut res: Vec<Box<dyn AstTree>> = vec![];
        let mut err: Option<String> = None;
        for element in &self.elements {
            if err.is_some() { break; }
            match element.parse(lexer, &mut res) {
                Ok(_) => {}
                Err(err_msg) => { err = Some(err_msg) }
            }
        }
        if err.is_none() { Ok(self.factory.make(res)) } else { Err(err.unwrap()) }
    }

    pub fn is_match(&self, lexer: &mut dyn Lexer) -> bool {
        if self.elements.len() > 0 { self.elements.get(0).unwrap().is_match(lexer) } else { false }
    }

    pub fn reset(mut self, factory: Option<Box<dyn AstFactory>>) -> Self {
        self.elements.clear();
        if let Some(factory) = factory {
            self.factory = factory;
        }
        self
    }

    pub fn reset_def(mut self) -> Self {
        self.reset(None)
    }


    pub fn number(mut self, factory: Option<Box<dyn AstLeafFactory>>) -> Self {
        self.elements.push(NumToken::new(factory));
        self
    }

    pub fn identifier(mut self, factory: Option<Box<dyn AstLeafFactory>>) -> Self {
        self.elements.push(IdToken::new(factory));
        self
    }

    pub fn string(mut self, factory: Option<Box<dyn AstLeafFactory>>) -> Self {
        self.elements.push(StrToken::new(factory));
        self
    }


    pub fn token(mut self, pat: Vec<&str>) -> Self {
        self.elements.push(Leaf::new(pat));
        self
    }

    pub fn sep(mut self, pat: Vec<&str>) -> Self {
        self.elements.push(Skip::new(pat));
        self
    }

    pub fn or(mut self, vec: Vec<&Rc<Parser>>) -> Self {
        let mut parser_rc_vec = vec![];
        for parser in vec {
            parser_rc_vec.push(Rc::clone(parser));
        }
        self.elements.push(OrTree::new(parser_rc_vec));
        self
    }

    pub fn maybe(mut self, factory: Option<Box<dyn AstFactory>>) -> Self {
        todo!("完成其他功能后实现，需要各个结构体均实现了AstFactory 底层结构均增加了 clone ")
    }

    pub fn option(mut self, repeat: Rc<Parser>) -> Self {
        self.elements.push(Repeat::new(repeat,true));
        self
    }

    pub fn repeat(mut self, repeat: Rc<Parser>) -> Self {
        self.elements.push(Repeat::new(repeat,false));
        self
    }

    pub fn expr(mut self, f: Box<dyn AstFactory>, factor: Rc<Parser>, operators: Rc<Operators>) -> Self {
        let expr = Expr::new(Rc::clone(&factor), operators, f);
        self.elements.push(expr);
        self
    }


    pub fn insert_choice(mut self, factory: Option<Box<dyn AstFactory>>) -> Self {
        todo!()
    }
}