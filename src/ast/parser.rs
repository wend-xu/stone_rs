use std::cell::RefCell;
use crate::ast::ast_tree::AstTree;
use crate::ast::element::{Element, Expr, IdToken, Leaf, NumToken, Operators, OrTree, Repeat, Skip, StrToken, Tree};
use crate::ast::factory::{AstFactory, AstLeafFactory, AstListFactory};
use crate::lexer::lexer::Lexer;
use std::rc::Rc;

// #[derive(Copy, Clone)]
pub struct Parser {
    factory: Box<dyn AstFactory>,
    elements: Vec<Box<dyn Element>>,
}

impl Parser {
    pub fn rule_def() -> Parser {
        let factory = AstListFactory::new();
        Self::rule(factory)
    }

    pub fn rule(factory: Box<dyn AstFactory>) -> Parser {
        Parser { factory, elements: vec![] }
    }


    pub fn parse(&self, lexer: &mut dyn Lexer) -> Result<Box<dyn AstTree>, String> {
        let mut res: Vec<Box<dyn AstTree>> = vec![];
        for element in &self.elements {
            element.parse(lexer, &mut res)?
        }

        Ok(self.factory.make(res))
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

    pub fn ast(mut self,pat: &Rc<RefCell<Self>>) -> Self{
        self.elements.push(Tree::new(pat));
        self
    }

    pub fn sep(mut self, pat: Vec<&str>) -> Self {
        self.elements.push(Skip::new(pat));
        self
    }

    pub fn or(mut self, vec: Vec<&Rc<RefCell<Parser>>>) -> Self {
        let mut parser_rc_vec = vec![];
        for parser in vec {
            parser_rc_vec.push(Rc::clone(parser));
        }
        self.elements.push(OrTree::new(parser_rc_vec));
        self
    }

    pub fn or_ref(&mut self, vec: Vec<&Rc<RefCell<Parser>>>) -> &Self {
        let mut parser_rc_vec = vec![];
        for parser in vec {
            parser_rc_vec.push(Rc::clone(parser));
        }
        self.elements.push(OrTree::new(parser_rc_vec));
        self
    }

    pub fn maybe(mut self, factory: Option<Box<dyn AstFactory>>) -> Self {
        todo!("需要的时候在实现，用于数组类型")
    }

    pub fn option(mut self, repeat: &Rc<RefCell<Parser>>) -> Self {
        self.elements.push(Repeat::new(repeat,true));
        self
    }

    pub fn repeat(mut self, repeat: &Rc<RefCell<Parser>>) -> Self {
        self.elements.push(Repeat::new(repeat,false));
        self
    }

    pub fn expr(mut self, f: Box<dyn AstFactory>, factor: &Rc<RefCell<Parser>>, operators: &Rc<Operators>) -> Self {
        let expr = Expr::new(Rc::clone(factor), Rc::clone(operators), f);
        self.elements.push(expr);
        self
    }

    pub fn expr_ref(&mut self, f: Box<dyn AstFactory>, factor: &Rc<RefCell<Parser>>, operators: &Rc<Operators>) -> &Self {
        let expr = Expr::new(Rc::clone(factor), Rc::clone(operators), f);
        self.elements.push(expr);
        self
    }

    pub fn insert_choice(mut self, factory: Option<Box<dyn AstFactory>>) -> Self {
        todo!("需要的时候再实现")
    }

    // todo 判断self 是 rc 还是 原始类型，决定 Rc::new 还是 Clone
    pub fn rc(mut self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }
}