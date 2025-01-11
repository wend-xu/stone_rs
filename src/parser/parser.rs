use std::any::TypeId;
use crate::ast::ast_tree::AstTree;
use crate::parser::element::{Element, Expr, IdToken, Leaf, NumToken, Operators, OrTree, Repeat, Skip, StrToken, Tree};
use crate::parser::factory::{AstFactory, AstLeafFactory, AstListFactory};
use crate::lexer::lexer::Lexer;
use std::cell::RefCell;
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

    pub fn identifier(mut self, factory: Option<Box<dyn AstLeafFactory>>, reserved: &Vec<&str>) -> Self {
        self.elements.push(IdToken::new(factory, &reserved));
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

    pub fn ast(mut self, pat: &Rc<RefCell<Self>>) -> Self {
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

    // maybe 跟 option 的区别在于，可选值为空时是否需要在语法树挂载节点
    pub fn maybe(mut self, repeat: &Rc<RefCell<Parser>>) -> Self {
        let factory_copy = repeat.borrow().factory.clone();

        let repeat = Rc::clone(repeat);
        let maybe = Parser::rule(factory_copy).rc();

        self.elements.push(OrTree::new(vec![repeat, maybe]));
        self
    }

    pub fn option(mut self, repeat: &Rc<RefCell<Parser>>) -> Self {
        self.elements.push(Repeat::new(repeat, true));
        self
    }

    pub fn repeat(mut self, repeat: &Rc<RefCell<Parser>>) -> Self {
        self.elements.push(Repeat::new(repeat, false));
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

    pub fn insert_choice(mut self, parser: &Rc<RefCell<Parser>>) -> Self {
        if let Some(mut ele_0) =  self.elements.get_mut(0) {
            if ele_0.el_actual_type_id() == TypeId::of::<OrTree>() {
                let mut or_tree =
                    ele_0.to_any_mut().downcast_mut::<OrTree>().unwrap();
                or_tree.insert(parser);
                return self;
            }
        }
        let otherwise = self.otherwise(parser);
        self.or(vec![parser,&otherwise])
    }

    fn otherwise(&mut self,parser:&Rc<RefCell<Parser>>) -> Rc<RefCell<Parser>> {
        let factory_clone = parser.borrow().factory.clone();
        Parser::rule(factory_clone).rc()
    }

    pub fn rc(mut self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self))
    }
}