#[cfg(test)]
mod eval_tests {
    use std::any::TypeId;
    use TokenValue::IDENTIFIER;
    use crate::ast::ast_leaf::{AstLeaf, IdentifierLiteral, NumberLiteral};
    use crate::ast::eval::{EvalRes, Evaluate};
    use crate::token::token_identifier::TokenIdentifier;
    use crate::ast::ast_tree::AstTree;
    use crate::ast::basic_parser::stone_parser;
    use crate::ast::environment::{EnvWrapper, MapEnv};
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
        let wrapper = EnvWrapper::new();
        let literal = IdentifierLiteral::new(TokenIdentifier::new(0, "i"));
        let res = literal.eval().do_eval(&wrapper).unwrap();
        println!("{:?}",res);
    }

    #[test]
    fn env_test(){
        let wrapper = EnvWrapper::new();
        let wrapper1 = EnvWrapper::new_with(MapEnv::new());
        let code = "i+1".to_string();
        let mut lexer = LineReaderLexer::new(code);
        let parser = stone_parser();
        let tree_res = parser.parse(&mut lexer);
        let tree = match tree_res {
            Ok(tree) => {
                tree
            }
            Err(msg) => {
                panic!("{}", msg);
            }
        };
        println!("{}", tree.location());
        println!("type {}", tree.child(0).unwrap().actual_type_id() == TypeId::of::<AstLeaf>());
        println!("type {}", tree.child(0).unwrap().actual_type_id() == TypeId::of::<IdentifierLiteral>());

        println!("type {}", tree.child(1).unwrap().actual_type_id() == TypeId::of::<AstLeaf>());
        println!("type {}", tree.child(1).unwrap().actual_type_id() == TypeId::of::<IdentifierLiteral>());

        println!("type {}", tree.child(2).unwrap().actual_type_id() == TypeId::of::<AstLeaf>());
        println!("type {}", tree.child(2).unwrap().actual_type_id() == TypeId::of::<NumberLiteral>());
        let eval = tree.eval();
        let eval_res_res = eval.do_eval(&wrapper);
        match eval_res_res {
            Ok(eval_res) => {
                println!("{:?}",eval_res)
            }
            Err(err) => {
                panic!("Eval error: {:?}", err);
            }
        }
    }
}