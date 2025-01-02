#[cfg(test)]
mod eval_tests {
    use std::any::TypeId;
    use TokenValue::IDENTIFIER;
    use crate::ast::ast_leaf::{AstLeaf, IdentifierLiteral, NumberLiteral};
    use crate::ast::ast_list::NullStmt;
    use crate::ast::eval::{EvalRes, Evaluate};
    use crate::token::token_identifier::TokenIdentifier;
    use crate::ast::ast_tree::AstTree;
    use crate::ast::basic_parser::stone_parser;
    use crate::ast::environment::{Env, EnvWrapper, MapEnv};
    use crate::lexer::lexer::Lexer;
    use crate::lexer::line_reader_lexer::LineReaderLexer;
    use crate::token::TokenValue;

    #[test]
    fn eval_test(){
        // let literal = IdentifierLiteral::new(TokenIdentifier::new(0,"+"));
        //
        // let x = literal.eval();
        // println!("a identifier literal eval while return : {:?}",x.do_eval());
        // let x1 = literal.eval();
        // println!("a identifier literal eval while return : {:?}",x1.do_eval());
        let value = IDENTIFIER("=".to_string());
        let eq = &value == "=";
        println!("{}", eq);
        let mut wrapper = EnvWrapper::new();
        let literal = IdentifierLiteral::new(TokenIdentifier::new(0, "i"));
        let res = literal.eval().do_eval(&mut wrapper).unwrap();
        println!("{:?}",res);
    }

    #[test]
    fn env_test(){
        let mut wrapper = EnvWrapper::new();
        let code = r#"
i = 2 ;
i = i+ " love u";
j = 6
j = j + 10 % 3
k = j * 3 +1-j/2*(j+1)
"#.to_string();
        _eval(code, &mut wrapper);
        println!(" i = {:?}",wrapper.get_ref("i").unwrap());
        println!(" j = {:?}",wrapper.get_ref("j").unwrap());
        println!(" k = {:?}",wrapper.get_ref("k").unwrap());
    }

    fn _eval(code:String, env:&mut EnvWrapper){
        let mut lexer = LineReaderLexer::new(code);
        let parser = stone_parser();
        while let Some(_) = lexer.peek(0) {
            let tree_res  = parser.parse(&mut lexer);
            let tree = match tree_res {
                Ok(tree) => {
                    tree
                }
                Err(msg) => {
                    panic!("{}", msg);
                }
            };
            let is_null_sata = tree.actual_type_id() == TypeId::of::<NullStmt>();
            println!("location:\n{}", tree.location());
            // println!("location:\n{}", is_null_sata);
            if is_null_sata {
                continue;
            }
            let eval = tree.eval();
            let eval_res_res = eval.do_eval(env);
            match eval_res_res {
                Ok(eval_res) => {
                    // println!("{:?}",eval_res);
                }
                Err(err) => {
                    panic!("Eval error: {:?}", err);
                }
            }
        }

    }
}