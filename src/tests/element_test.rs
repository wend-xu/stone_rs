#[cfg(test)]
mod element_tests {
    use std::any::{Any, TypeId};
    use crate::ast::ast_list::AstList;
    use crate::ast::ast_tree::AstTree;
    use crate::ast::element::{Element, IdToken, Leaf};
    use crate::lexer::line_reader_lexer::LineReaderLexer;
    use crate::token::Token;
    use crate::util::type_util::struct_is_type;

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
}