#[cfg(test)]
mod eq_tests {
    use std::ops::Deref;
    use crate::ast::ast_leaf::AstLeaf;
    use crate::ast::leaf::identifier_literal::IdentifierLiteral;
    use crate::ast::leaf::number_literal::NumberLiteral;
    use crate::ast::leaf::string_literal::StringLiteral;
    use crate::eval::environment::EnvWrapper;
    use crate::eval::eval::EvalRes;
    use crate::lexer::lexer::Lexer;
    use crate::lexer::line_reader_lexer::LineReaderLexer;
    use crate::parser::basic_parser_macros::stone_parser;
    use crate::token::{Token, TokenLine, TokenValue};
    use crate::token::token_end::{TokenEOF, TokenEOL};
    use crate::token::token_identifier::TokenIdentifier;
    use crate::token::token_number::TokenNumber;
    use crate::token::token_string::TokenString;

    #[test]
    fn token_value_eq_test() {
        assert_eq!(TokenValue::EOL == TokenValue::EOL, true);
        assert_eq!(TokenValue::EOF == TokenValue::EOF, true);
        assert_eq!(TokenValue::IDENTIFIER("id".to_string()) == TokenValue::IDENTIFIER("id".to_string()), true);
        assert_eq!(TokenValue::NUMBER(20) == TokenValue::NUMBER(20), true);
        assert_eq!(TokenValue::StringVal("id".to_string()) == TokenValue::StringVal("id".to_string()), true);


        assert_eq!(TokenValue::EOL == TokenValue::EOF, false);
        assert_eq!(TokenValue::IDENTIFIER("id".to_string()) == TokenValue::IDENTIFIER("i2".to_string()), false);
        assert_eq!(TokenValue::NUMBER(20) == TokenValue::NUMBER(10), false);
        assert_eq!(TokenValue::StringVal("id".to_string()) == TokenValue::StringVal("id_o".to_string()), false);

        assert_eq!(TokenLine::new(10) == TokenLine::new(10), true);
        assert_eq!(TokenLine::new(11) == TokenLine::new(10), false);
    }

    #[test]
    fn token_eq_test() {
        let eol_1 = TokenEOL::new(1);
        let eol_2 = TokenEOL::new(2);
        let eol_3 = TokenEOL::new(2);
        assert_eq!(eol_1 == eol_2, false);
        assert_eq!(eol_3 == eol_2, true);

        let eof_1 = TokenEOF::new(99);
        let eof_2 = TokenEOF::new(99);
        let eof_3 = TokenEOF::new(999);
        assert_eq!(eof_1 == eof_2, true);
        assert_eq!(eof_1 == eof_3, false);

        let identifier_1 = TokenIdentifier::new(1, "d");
        let identifier_2 = TokenIdentifier::new(1, "d");
        let identifier_3 = TokenIdentifier::new(2, "d");
        let identifier_4 = TokenIdentifier::new(1, "d4");
        assert_eq!(identifier_1 == identifier_2, true);
        assert_eq!(identifier_1 == identifier_3, false);
        assert_eq!(identifier_1 == identifier_4, false);

        let number_1 = TokenNumber::new(1, 1);
        let number_2 = TokenNumber::new(1, 1);
        let number_3 = TokenNumber::new(2, 1);
        let number_4 = TokenNumber::new(2, 4);
        assert_eq!(number_1 == number_2, true);
        assert_eq!(number_1 == number_3, false);
        assert_eq!(number_1 == number_4, false);


        let str_1 = TokenString::new(1, "d");
        let str_2 = TokenString::new(1, "d");
        let str_3 = TokenString::new(2, "d");
        let str_4 = TokenString::new(1, "d4");
        assert_eq!(str_1 == str_2, true);
        assert_eq!(str_1 == str_3, false);
        assert_eq!(str_1 == str_4, false);

        let vec: Vec<Box<dyn Token>> = vec![str_1, str_2, str_3];
        assert_eq!(&vec[0] == &vec[1], true);
        assert_eq!(&vec[0] == &vec[2], false);
    }

    #[test]
    fn ast_leaf_eq_test() {
        let leaf_1 = AstLeaf::new(TokenString::new(1, "d"));
        let leaf_2 = AstLeaf::new(TokenString::new(1, "d"));
        let leaf_3 = AstLeaf::new(TokenString::new(1, "d2"));
        let leaf_4 = leaf_1.clone();
        let leaf_5 = leaf_3.clone();
        assert_eq!(leaf_1 == leaf_2, true);
        assert_eq!(leaf_1 == leaf_3, false);
        assert_eq!(leaf_1 == leaf_4, true);
        assert_eq!(leaf_1 == leaf_5, false);

        let identifier_literal_1 = IdentifierLiteral::new(TokenIdentifier::new(1, "1"));
        let identifier_literal_2 = IdentifierLiteral::new(TokenIdentifier::new(1, "1"));
        let identifier_literal_3 = IdentifierLiteral::new(TokenIdentifier::new(1, "2"));
        let identifier_literal_4 = identifier_literal_2.clone();
        let identifier_literal_5 = identifier_literal_3.clone();
        assert_eq!(identifier_literal_1 == identifier_literal_2, true);
        assert_eq!(identifier_literal_1 == identifier_literal_3, false);
        assert_eq!(identifier_literal_1 == identifier_literal_4, true);
        assert_eq!(identifier_literal_1 == identifier_literal_5, false);


        let identifier_number_1 = NumberLiteral::new(TokenNumber::new(1, 1));
        let identifier_number_2 = NumberLiteral::new(TokenNumber::new(1, 1));
        let identifier_number_3 = NumberLiteral::new(TokenNumber::new(2, 1));
        let identifier_number_4 = identifier_number_2.clone();
        let identifier_number_5 = identifier_number_3.clone();
        assert_eq!(identifier_number_1 == identifier_number_2, true);
        assert_eq!(identifier_number_1 == identifier_number_3, false);
        assert_eq!(identifier_number_1 == identifier_number_4, true);
        assert_eq!(identifier_number_1 == identifier_number_5, false);


        let str_literal_1 = StringLiteral::new(TokenString::new(1, "1"));
        let str_literal_2 = StringLiteral::new(TokenString::new(1, "1"));
        let str_literal_3 = StringLiteral::new(TokenString::new(1, "2"));
        let str_literal_4 = str_literal_2.clone();
        let str_literal_5 = str_literal_3.clone();
        assert_eq!(str_literal_1 == str_literal_2, true);
        assert_eq!(str_literal_1 == str_literal_3, false);
        assert_eq!(str_literal_1 == str_literal_4, true);
        assert_eq!(str_literal_1 == str_literal_5, false);
    }

    #[test]
    fn ast_list_eq_test() {
        let mut wrapper = EnvWrapper::new();
        let code = r#"
i = 2 ;
i = i+ " love u";
j = -6
j = j + 10 % 3
k = j * 3 +1-j/2*(j+1)
if j < -1 {
    l = 10
} else {
    l = 5
}
"#;
        let mut lexer = LineReaderLexer::new(code.to_string());
        let mut lexer_2 = LineReaderLexer::new(code.to_string());
        let parser = stone_parser();
        let mut res = EvalRes::VOID;
        while let Some(_) = lexer.peek(0) {
            let tree_res = parser.parse(&mut lexer).unwrap();
            println!("{}", tree_res.location());
            let tree_res_2 = parser.parse(&mut lexer_2).unwrap();
            println!("{}", tree_res_2.location());
            println!("{}", tree_res.location() == tree_res_2.location());
            let x = tree_res_2.deref();
            println!("{:?}", tree_res.actual_type_id());
            println!("{:?}", x.actual_type_id());
            // println!("eq->   {}",tree_res.eq_tree(x));
            assert_eq!(tree_res.eq_tree(x), true);
        }
    }
}