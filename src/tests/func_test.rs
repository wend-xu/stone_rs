use crate::lexer::lexer::Lexer;

#[cfg(test)]
pub mod eval_test {
    use crate::ast::ast_list::AstList;
    use crate::ast::ast_tree::AstTree;
    use crate::lexer::lexer::Lexer;
    use crate::lexer::line_reader_lexer::LineReaderLexer;
    use crate::parser::func_parser::stone_parser_with_func;
    use crate::parser::parser::Parser;
    use crate::token::TokenValue;
    use crate::{or, seq};
    use crate::eval::environment::{Env, EnvWrapper};
    use crate::eval::eval::{EvalRes, Evaluate};

    #[test]
    pub fn def_match_test() {
        let reserved = vec!["}", ";", TokenValue::literal_eol(),")"];
        let param = seq!(seq: id->reserved);
        let params = seq!(param_list: param { "," param });
        // 注意这里展开后使用的是 maybe
        let param_list = seq!(seq: "(" [ params ]* ")");
        let def = seq!(def: "def" id->reserved param_list);
        let parser = or!(no_rc: param_list  , seq!(null:(";","\n")) );


        let code = "(aa,b)";

        let mut lexer = LineReaderLexer::new(code.to_string());
        println!("{}",parser.is_match(&mut lexer));
        _p_res(&mut lexer,&parser);
        // let mut res:Vec<Box<dyn AstTree>> = vec![];
        // let literal = IdentifierLiteral::new(TokenIdentifier::new(0, "a"));
        // res.push(literal);
        // let pl = ParameterListFactory::new().make(res);
        // println!("pl: \n{}", pl.location());

    }

    #[test]
    pub fn func_tree_test() {
        let code = r#"
            def hahha ( aaa ){
                even = 1
            }
        "#;
        let mut wrapper = EnvWrapper::new();
        let mut lexer = LineReaderLexer::new(code.to_string());
        let parser = stone_parser_with_func();
        let list = _p_res(&mut lexer, &parser);

        let res = list.child(1).unwrap().eval().do_eval(&mut wrapper);
        println!("{:?}", res);
        let func_name = res.unwrap();
        match  wrapper.get(func_name.to_string().as_str()).unwrap() {
            EvalRes::FUNCTION(fun_name, param_list , block) => {
                println!("函数名称：{}",fun_name);
                println!("代码块：\n{}",block.location());
                let eval_res = block.do_eval(&mut wrapper);
                println!("{:?}",eval_res);
            }
            _ => {}
        }


    }


    #[test]
    pub fn func_tree_clone_test() {
        let code = "even = 1";
        let mut wrapper = EnvWrapper::new();
        let mut lexer = LineReaderLexer::new(code.to_string());
        let parser = stone_parser_with_func();
        let list = _p_res(&mut lexer, &parser);

        let res = list.child(0).unwrap();
        println!("\n\n\n\n\n\n\n{} \n\n\n\n\n\n\n",res.location());
        let x = (*res).clone_tree();
        println!("\n\n\n\n\n\n\n{} \n\n\n\n\n\n\n",x.location());

        let ast_list = AstList::new("aaaa", vec![x]);
        println!("\n\n\n\n\n\n\n{} \n\n\n\n\n\n\n",ast_list.location());
        println!("{}",ast_list.clone().location());

    }

    fn _p_res(lexer: &mut dyn Lexer, parser: &Parser) -> AstList {
        let mut err = None;
        let mut ast_tree_vec: Vec<Box<dyn AstTree>> = vec![];
        while let Some(token) = lexer.peek(0) {
            if TokenValue::EOF.eq(token.value()) || err.is_some() {
                break;
            }
            if !parser.is_match(lexer) {
                err = Some(Err(format!("无法处理的token： {:?}", lexer.read().unwrap().value())));
                break;
            }
            let res_pro = parser.parse(lexer);
            if res_pro.is_ok() {
                let res = res_pro.unwrap();
                // println!("{}", res.location());
                ast_tree_vec.push(res);
            } else {
                err = Some(res_pro);
            }
        }
        if err.is_some() {
            let err = err.unwrap();
            if let Err(err_msg) = err {
                println!("{}", err_msg);
            }
            panic!()
        }
        let res_ast_tree = AstList::new_def(ast_tree_vec);
        println!("{}", res_ast_tree.location());
        res_ast_tree
    }
}