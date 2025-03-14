use crate::lexer::lexer::Lexer;

#[cfg(test)]
pub mod eval_test {
    use std::any::TypeId;
    use crate::ast::ast_list::AstList;
    use crate::ast::ast_tree::AstTree;
    use crate::eval::environment::{Env, EnvWrapper, MapNestedEnv};
    use crate::eval::eval::{EvalRes, Evaluate};
    use crate::lexer::lexer::Lexer;
    use crate::lexer::line_reader_lexer::LineReaderLexer;
    use crate::parser::func_parser::stone_parser_with_func;
    use crate::parser::parser::Parser;
    use crate::token::TokenValue;
    use crate::{or, seq};
    use crate::ast::list::null_stmt::NullStmt;
    use crate::stone::StoneLang;

    #[test]
    pub fn def_match_test() {
        let reserved = vec!["}", ";", TokenValue::literal_eol(), ")"];
        let param = seq!(seq: id->reserved);
        let params = seq!(param_list: param { "," param });
        // 注意这里展开后使用的是 maybe
        let param_list = seq!(seq: "(" [ params ]* ")");
        let def = seq!(def: "def" id->reserved param_list);
        let parser = or!(no_rc: param_list  , seq!(null:(";","\n")) );


        let code = "(aa,b)";

        let mut lexer = LineReaderLexer::new_with_code(code.to_string());
        println!("{}", parser.is_match(&mut lexer));
        _p_res(&mut lexer, &parser);
        // let mut res:Vec<Box<dyn AstTree>> = vec![];
        // let literal = IdentifierLiteral::new(TokenIdentifier::new(0, "a"));
        // res.push(literal);
        // let pl = ParameterListFactory::new().make(res);
        // println!("pl: \n{}", pl.location());

    }

    #[test]
    pub fn func_tree_test() {
        let code = r#"
            def hahha ( aaa,bbb ){
                even = 1
            }

            hahha(aa,b)(c,d)
        "#;
        let mut wrapper = EnvWrapper::new();
        let mut lexer = LineReaderLexer::new_with_code(code.to_string());
        let parser = stone_parser_with_func();
        let list = _p_res(&mut lexer, &parser);

        let res = list.child(1).unwrap().eval().do_eval(&mut wrapper);
        println!("{:?}", res);
        let func_name = res.unwrap();
        match wrapper.get(func_name.to_string().as_str()).unwrap() {
            EvalRes::FUNCTION(fun_name, param_list, block) => {
                println!("函数名称：{}", fun_name.unwrap_or("匿名函数".to_string()));
                println!("代码块：\n{}", block.location());
                let eval_res = block.do_eval(&mut wrapper);
                println!("{:?}", eval_res);
            }
            _ => {}
        }
    }


    #[test]
    pub fn func_tree_clone_test() {
        let code = "even = 1";
        let mut wrapper = EnvWrapper::new();
        let mut lexer = LineReaderLexer::new_with_code(code.to_string());
        let parser = stone_parser_with_func();
        let list = _p_res(&mut lexer, &parser);

        let res = list.child(0).unwrap();
        println!("\n\n\n\n\n\n\n{} \n\n\n\n\n\n\n", res.location());
        let x = (*res).clone_tree();
        println!("\n\n\n\n\n\n\n{} \n\n\n\n\n\n\n", x.location());

        let ast_list = AstList::new("aaaa", vec![x]);
        println!("\n\n\n\n\n\n\n{} \n\n\n\n\n\n\n", ast_list.location());
        println!("{}", ast_list.clone().location());
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

    #[test]
    fn test_function() {
        let code = r#"
            def add_one(add){
                add = add+1
            }
            add = 1;
            add_one(add)
        "#;
        test_2_tree(code.to_string());
    }


    #[test]
    fn test_function_2() {
        let code = r#"
            def add_one(add){
                add = add+1;
                add
            }
            add = 65535;
            add = add_one(add);
            add
        "#;
        test_eval(code.to_string());
    }


    #[test]
    fn test_function_3() {
        let code = r#"
            def fib(n){
               if n < 2 {
                  n
               }else {
                  fib(n - 1) + fib ( n -2 )
               }
            }
            fib(10);
            fib(1);
        "#;
        test_eval(code.to_string());
    }

    #[test]
    fn test_function_4() {
        let code = r#"
            def add_one(){
                add = add+1;
                add
            }
            add = 65535;
            add_one();
            add
        "#;
        test_eval(code.to_string());
    }

    #[test]
    fn test_nest_env() {
        let mut outer = EnvWrapper::new();
        outer.put("a".to_string(), EvalRes::BOOLEAN(false));

        {
            let mut nest = MapNestedEnv::new_with(&mut outer);
            let mut wrapper = nest.wrapper();

            match wrapper.put("a".to_string(), EvalRes::BOOLEAN(true)) {
                Ok(ok) => {}
                Err(err) => {
                    println!("{}", err);
                }
            };
        }

        println!("{:?}", outer.get_ref("a").unwrap());
    }


    fn test_eval(code:String){
        let mut stone = StoneLang::new_def();
        match stone.code_eval(code.to_string()) {
            Ok(env) => {
                // println!("执行成功:{:?}", env.get_ref("add").unwrap());
            }
            Err(err_msg) => {
                println!("{}", err_msg);
            }
        };
    }

    fn test_2_tree(code:String){
        let mut stone = StoneLang::new_def();
        match stone.code_2_ast(code.to_string()) {
            Ok(env) => {
                println!("{}",AstList::new_def(env).location() );
            }
            Err(err_msg) => {
                println!("{}", err_msg);
            }
        };
    }


    #[test]
    fn test_function_5() {
        let code = r#"
            add_one = fun(add){
                add = add+1;
                add
            }
            i = add_one(1)
            i
        "#;
        test_eval(code.to_string());
    }


    #[test]
    fn test_function_6() {
        let code = r#"
           def add_fun_def(){
               fun(to_add) { to_add + 1  }
           }
           add_fun = add_fun_def();
           add_fun;
           i = add_fun(1)
           i
        "#;
        test_eval(code.to_string());
    }
}