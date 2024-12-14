use std::rc::Rc;
use crate::ast::ast_tree::{AstFactory, AstTree, BinaryExprFactory};
use crate::ast::element::{Element, Expr, Operators, OrTree};
use crate::lexer::lexer::Lexer;

pub struct Parser {
    elements: Vec<Box<dyn Element>>,
}

pub fn ast_node_factory(res: &mut Vec<Box<dyn AstTree>>) -> Box<dyn AstTree> {
    todo!()
}

// pub fn ast_node_factory_gen<T:AstTree>(res: &mut Vec<Box<dyn AstTree>>) ->  Box<dyn AstTree> {
//     type tree_type = T;
//     let option = tree_type::gen();
//     todo!()
// }

impl Parser {
    pub fn new() -> Parser {
        Parser { elements: vec![] }
    }

    pub fn new_with_elements(elements: Vec<Box<dyn Element>>) -> Parser {
        Parser { elements }
    }


    pub fn rule() -> Self {
        Parser {
            elements: vec![]
        }
    }

    pub fn parse(&self, lexer: &mut dyn Lexer) -> Result<Box<dyn AstTree>,String>{
        let mut res: Vec<Box<dyn AstTree>> = vec![];
        let mut err: Option<String> = None;
        for element in &self.elements {
            if err.is_some() { break }
            match element.parse(lexer, &mut res){
                Ok(_) => {}
                Err(err_msg) => { err = Some(err_msg) }
            }
        }
        if err.is_none() { Ok(ast_node_factory(&mut res)) } else { Err(err.unwrap()) }
    }

    pub fn is_match(&self, lexer: &mut dyn Lexer) -> bool {
        if self.elements.len() > 0 { self.elements.get(0).unwrap().is_match(lexer) } else { false }
    }

    // f可能是为引用类型实现，这是编译器所无法确定的
    // 如此时将一个 Box<F> 存入 Vec<Box<dyn Element>> ，就可能会出现引用类型被提前释放的情况
    // 在使用泛型的情况下，最简单的方式是添加 'static 注解代表传入是引用类型时必须是 static 生命周期
    //
    // rust 无法限制 F 对应的实现不能为引用类型 ，如 实现  impl AstFactory for &BinaryExprFactory
    // 若传入的是对引用的实现，不添加生命周期标识，可能会造成悬垂指针，这是rust编译器所不允许的
    // 若添加标识，则至少是与self的生命周期一般长
    pub fn expr<F: AstFactory+'static>(mut self,f: F, factor: Rc<Parser>, operators: Rc<Operators>) -> Self {
        let expr = Expr::new(Rc::clone(&factor), operators, Box::new(f));
        self.elements.push(Box::new(expr));
        self
    }

    pub fn or(mut self,vec: Vec<&Rc<Parser>>) -> Self{
        let mut parser_rc_vec = vec![];
        for parser in vec {
            parser_rc_vec.push(Rc::clone(parser));
        }
        self.elements.push(Box::new(OrTree::new(parser_rc_vec)));
        self
    }
}