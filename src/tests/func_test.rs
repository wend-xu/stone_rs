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
        let mut lexer = LineReaderLexer::new(code.to_string());
        let parser = stone_parser_with_func();
        _p_res(&mut lexer, &parser);
    }

    fn _p_res(lexer: &mut dyn Lexer, parser: &Parser) {
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
            return;
        }
        let res_ast_tree = AstList::new_def(ast_tree_vec);
        println!("{}", res_ast_tree.location());
    }
}