#[cfg(test)]
mod element_tests {
    use crate::ast::ast_list::AstList;
    use crate::ast::ast_tree::{AstFactory, AstTree, BinaryExprFactory};
    use crate::ast::element::{Element, IdToken, Leaf, NumToken, Operators, OrTree, Skip, StrToken};
    use crate::ast::parser::Parser;
    use crate::lexer::line_reader_lexer::LineReaderLexer;
    use std::any::{Any, TypeId};
    use std::rc::Rc;
    use crate::lexer::lexer::Lexer;

    #[test]
    fn match_test() {
        let ast_list = AstList::new(vec![]);

        let mut vec_dyn_token: Vec<Box<dyn AstTree>> = vec![Box::new(ast_list)];

        let box_token = vec_dyn_token.get(0).unwrap();

        println!("is Ast List : {}", box_token.actual_type_id() == TypeId::of::<AstList>());
        println!("is Ast List : {}", box_token.type_id() == TypeId::of::<AstList>());
        // println!("is Ast List : {}",struct_is_type()::<AstList>(box_token));
        // println!("is Ast List : {}",box_token.actual_type_id() == TypeId::of::<AstList>());
    }

    #[test]
    fn token_test() {
        let code = "code".to_string();
        let mut lexer = LineReaderLexer::new(code);
        let x = IdToken;
        let mut res: Vec<Box<dyn AstTree>> = vec![];
        x.parse(&mut lexer, &mut res);
        println!("{}", res.get(0).unwrap().location());
    }

    #[test]
    fn leaf_new(){
        let leaf = Leaf::new(vec!["(", ")"]);
        println!("{:?}", leaf);
        let code = "(".to_string();
        let mut lexer = LineReaderLexer::new(code);
        println!("{}", leaf.is_match(&mut lexer));
    }

    #[test]
    fn parser_with_generic(){
        let factor = Rc::new(Parser::rule());
        let operators = Rc::new( Operators::new());
        let x = Box::new(BinaryExprFactory{});

        // Parser::expr(BinaryExprFactory{},factor,operators);
        // let x1 = Parser::rule().expr::<BinaryExprFactory>(BinaryExprFactory {}, factor, operators);
        // let expr = Rc::new(x1);
        //
        // let parser = Parser::rule().or(vec![&expr]);

        let mut test =Test{vec:vec![]};
        test.add_not_generic(Box::new(OrTree::new(vec![])));
        let num_token = NumToken;
        let mut lexer = LineReaderLexer::new("111".to_string());
        let ref_num_token = &NumToken;
        // println!("match {}",num_token.is_match(&mut lexer));
        println!("match {}",ref_num_token.is_match(&mut lexer));
    }

    struct Test{
        vec:Vec<Box<dyn Element>>,
    }

    impl Test {
        pub fn add_not_generic(mut self,ele:Box<dyn Element>) -> Self{
            self.vec.push(ele);
            self
        }

        pub fn add_with_generic<E:Element+'static>(mut self,e:Box<E>)->Self{
            self.vec.push(e);
            self
        }
    }
}