use crate::ast::ast_tree::{AstFactory, AstTree};
use crate::ast::element::Element;
use crate::lexer::lexer::Lexer;

pub struct Parser {
    elements: Vec<Box<dyn Element>>,
}

pub fn ast_node_factory(res: &mut Vec<Box<dyn AstTree>>) ->  Box<dyn AstTree> {
    todo!()
}

// pub fn ast_node_factory_gen<T:AstTree>(res: &mut Vec<Box<dyn AstTree>>) ->  Box<dyn AstTree> {
//     type tree_type = T;
//     let option = tree_type::gen();
//     todo!()
// }

impl Parser {
    pub fn parse(&self, lexer: &mut dyn Lexer) -> Box<dyn AstTree> {
        let mut res: Vec<Box<dyn AstTree>> = vec![];
        for element in &self.elements {
            element.parse(lexer, &mut res);
        }
        ast_node_factory(&mut res)
    }

    pub fn is_match(&self, lexer: &mut dyn Lexer) -> bool {
        if self.elements.len() > 0 { self.elements.get(0).unwrap().is_match(lexer) } else { false }
    }
}