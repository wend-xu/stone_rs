#[cfg(test)]
mod element_tests {
    use crate::ast::ast_leaf::{IdentifierLiteral, StringLiteral};
    use crate::ast::ast_list::AstList;
    use crate::ast::ast_tree::AstTree;
    use crate::ast::basic_parser::stone_parser;
    use crate::ast::element::{Element, IdToken, Leaf, NumToken, Operators, OrTree, Precedence};
    use crate::ast::factory::{BinaryExprFactory, IdentifierLiteralFactory};
    use crate::ast::parser::Parser;
    use crate::lexer::line_reader_lexer::LineReaderLexer;
    use crate::token::token_identifier::TokenIdentifier;
    use crate::token::token_string::TokenString;
    use crate::util::str_util::{lines_concat_with_divide, wrapper_node_name, wrapper_sub_block};
    use std::any::{Any, TypeId};
    use std::fmt::format;
    use std::rc::Rc;
    use crate::lexer::lexer::Lexer;
    use crate::token::TokenValue;

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
        let x = IdToken::new(Some(factory),vec![]);
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
while i < 10 {
	if i % 2 == 0 {
		even = even + i
	}else {
		odd = odd + i
	}
	i = i + 1
}
        ";

        let mut lexer = LineReaderLexer::new(code.to_string());
        println!("分词完成");
        let parser = stone_parser();
        println!("语法解析器完成");
        let is_match = parser.is_match(&mut lexer);
        let res = if is_match {
            println!("开始构建语法树");
            parser.parse(&mut lexer)
        } else {
            Err("不匹配语法".to_string())
        };

        println!("构建语法树完成");
        match res {
            Ok(astTree) => {
                println!("{}", astTree.location())
            }
            Err(err_msg) => { println!("语法构建错误： {}", err_msg); }
        }
    }

    #[test]
    pub fn concat_test() {
        let str_vec = vec![
            "column1row1\ncolumn1row2".to_string(),
            "column2row1\ncolumn2row2\ncolumn2row3".to_string(),
            "column3row1\ncolumn3row2".to_string(),
            "column4row1\ncolumn4row2\ncolumn4row3".to_string(),
            "column5row1\ncolumn5row2".to_string(),
            // "第三列第一行".to_string(),
            // "第四列第一行\n第四列第二行".to_string(),
        ];
        println!("{}", lines_concat_with_divide(str_vec, Some("    ")));
    }


    #[test]
    pub fn concat_test_2() {
        let string_1 = StringLiteral::new(TokenString::new(1, "hello"));
        let string_2 = IdentifierLiteral::new(TokenIdentifier::new(1, "+"));
        let string_3 = StringLiteral::new(TokenString::new(1, "world"));
        let block = lines_concat_with_divide(vec![string_1.location(), string_2.location(), string_3.location()], Some("    "));

        let root = wrapper_node_name("ast_list".to_string());
        // println!("{}\n\n\n",root);
        let block = wrapper_sub_block(root, block);

        println!("{}", block);
    }


    #[test]
    pub fn concat_test_3() {
        let leaf = Leaf::new(vec!["(", ")", "\n"]);
        println!("{}", leaf.is_match(&mut LineReaderLexer::new("\n".to_string())));
        let mut operators = Operators::new();
        operators.add("=", Precedence::right(1));
        operators.add("==", Precedence::left(2));
        operators.add(">", Precedence::left(2));
        operators.add("<", Precedence::left(2));
        operators.add("+", Precedence::left(3));
        operators.add("-", Precedence::left(3));
        operators.add("*", Precedence::left(4));
        operators.add("/", Precedence::left(4));
        operators.add("%", Precedence::left(4));
        println!("{}",operators.get("==").is_some());
        println!("{}",operators.get("==").is_some());
    }

    #[test]
    fn parer_test_full() {
        let code = "
while i < 10 {
if i % 2 == 0 {
    even = even + i
}else {
    odd = odd + i
}
i = i + 1
}
        ";

        let mut lexer = LineReaderLexer::new(code.to_string());
        println!("分词完成");
        let parser = stone_parser();
        println!("语法解析器完成");
        _p_res(&mut lexer,&parser) ;
    }

    #[test]
    fn if_else_test() {
        let code = "
}
        ";

        let mut lexer = LineReaderLexer::new(code.to_string());
        println!("分词完成 \n {}",lexer);
        let parser = stone_parser();
        println!("语法解析器完成");
        _p_res(&mut lexer,&parser) ;
    }

    #[test]
    fn if_else_test_2() {
        let parser = Parser::rule_def().sep(vec!["else"]);
        let mut lexer = LineReaderLexer::new("else else if else".to_string());
        _p_res(&mut lexer,&parser)
    }

    fn _p_res(lexer:&mut dyn Lexer, parser:&Parser) {
        let mut err = None;
        let mut ast_tree_vec: Vec<Box<dyn AstTree>> = vec![];
        while let Some(token) = lexer.peek(0) {
            if TokenValue::EOF.eq(token.value()) || err.is_some() {
                break;
            }
            if !parser.is_match(lexer) {
                err = Some(Err(format!("无法处理的token： {:?}",lexer.read().unwrap().value())));
                break;
            }
            let res_pro = parser.parse(lexer);
            if res_pro.is_ok() {
                let res = res_pro.unwrap();
                ast_tree_vec.push(res);
            } else {
                err = Some(res_pro);
            }
        }
        if err.is_some() {
            let err = err.unwrap();
            if let Err(err_msg) = err{
                println!("{}", err_msg);
            }
            return;
        }
        let res_ast_tree = AstList::new(ast_tree_vec);
        println!("{}", res_ast_tree.location());
    }
}