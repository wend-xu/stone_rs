#[cfg(test)]
mod element_tests {
    use crate::ast::ast_list::AstList;
    use crate::ast::ast_tree::{AstTree};
    use crate::ast::element::{Element, IdToken, Leaf, NumToken, Operators, OrTree};
    use crate::ast::parser::Parser;
    use crate::lexer::line_reader_lexer::LineReaderLexer;
    use std::any::{Any, TypeId};
    use std::marker::PhantomData;
    use std::rc::Rc;
    use crate::ast::basic_parser::stone_parser;
    use crate::ast::factory::{AstFactory, BinaryExprFactory, IdentifierLiteralFactory};

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
        let factory = IdentifierLiteralFactory::new();
        let x = IdToken::new(Some(factory));
        let mut res: Vec<Box<dyn AstTree>> = vec![];
        x.parse(&mut lexer, &mut res);
        println!("{}", res.get(0).unwrap().location());
    }

    #[test]
    fn leaf_new() {
        let leaf = Leaf::new(vec!["(", ")"]);
        println!("{:?}", leaf);
        let code = "(".to_string();
        let mut lexer = LineReaderLexer::new(code);
        println!("{}", leaf.is_match(&mut lexer));
    }


    #[test]
    fn parser_with_generic() {
        let factor = Rc::new(Parser::rule_def());
        let operators = Rc::new(Operators::new());
        let x = Box::new(BinaryExprFactory {});

        // let x1 = Parser::rule_def().expr(BinaryExprFactory::new(), factor, operators);
    }

    #[test]
    fn parser_with_generic_2() {
        let factor = Rc::new(Parser::rule_def());
        let operators = Rc::new(Operators::new());
        let x = BinaryExprFactory::new();

        // Parser::expr(BinaryExprFactory{},factor,operators);
        // let x1 = Parser::rule().expr::<BinaryExprFactory>(BinaryExprFactory {}, factor, operators);
        // let expr = Rc::new(x1);
        //
        // let parser = Parser::rule().or(vec![&expr]);

        let mut test = Test { vec: vec![] };
        test.add_not_generic(OrTree::new(vec![]));
        let num_token = NumToken::new(None);
        let mut lexer = LineReaderLexer::new("111".to_string());
        let ref_num_token = &NumToken::new(None);
        // println!("match {}",num_token.is_match(&mut lexer));
        println!("match {}", ref_num_token.is_match(&mut lexer));
    }

    struct Test {
        vec: Vec<Box<dyn Element>>,
    }

    impl Test {
        pub fn add_not_generic(mut self, ele: Box<dyn Element>) -> Self {
            self.vec.push(ele);
            self
        }

        pub fn add_with_generic<E: Element + 'static>(mut self, e: Box<E>) -> Self {
            self.vec.push(e);
            self
        }
    }

    fn return_result(i: usize) -> Result<String, String> {
        if i % 5 == 0 { Err("i % 5 not 0".to_string()) } else { Ok(format!("i is :{}", i)) }
    }

    fn for_result() -> Result<String, String> {
        for i in 1..10 {
            println!("for {}", return_result(i)?);
        }
        Ok("loop finish".to_string())
    }

    #[test]
    fn result_test() {
        match for_result() {
            Ok(ok_msg) => { println!("{}", ok_msg) }
            Err(err_msg) => { println!("{}", err_msg); }
        };
    }


    #[test]
    fn parer_test() {
        let code = "\
even = 0
odd = 0
i  = 1
while i < 10 {
	if i % 2 == 0 {
		even = even + i
	}else {
		odd = odd + i
	}
	i = i + 1
}
even + odd
        ";

        let mut lexer = LineReaderLexer::new(code.to_string());
        println!("分词完成");
        let parser = stone_parser();
        println!("语法解析器完成");
        let is_match = parser.is_match(&mut lexer);
        let res = if is_match {
            println!("开始构建语法树");
            parser.parse(&mut lexer)
        }else {
            Err("不匹配语法".to_string())
        };

        println!("构建语法树完成");
        match res{
            Ok(astTree) => {
                println!("{}", astTree.location())
            }
            Err(err_msg ) => { println!("语法构建错误： {}", err_msg); }
        }
    }
}