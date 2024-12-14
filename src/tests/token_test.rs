#[cfg(test)]
mod token_tests {
    use regex::Regex;
    use crate::token::TokenValue;

    #[test]
    fn code_test() {
        let re = Regex::new(r"[+\-*/%=\\|&,.!?(){}\[\]]").unwrap();
        println!("{}", re.is_match("+"));
        println!("{}", re.is_match("-"));
        println!("{}", re.is_match("*"));
        println!("{}", re.is_match("/"));
        println!("{}", re.is_match("%"));
        println!("{}", re.is_match("="));
        println!("{}", re.is_match("\\"));
        println!("{}", re.is_match("|"));
        println!("{}", re.is_match("&"));
        println!("{}", re.is_match(","));
        println!("{}", re.is_match("."));
        println!("{}", re.is_match("!"));
        println!("{}", re.is_match("?"));
        println!("{}", re.is_match("("));
        println!("{}", re.is_match(")"));
        println!("{}", re.is_match("["));
        println!("{}", re.is_match("]"));
        println!("{}", re.is_match("{"));
        println!("{}", re.is_match("}"));
        println!("{}", re.is_match("【"));
        println!("{}", re.is_match("===="));

        // println!("括号匹配 {}",Regex::new(r"[A-Z_a-z][A-Z_a-z0-9]*|[+\-*/%=\\|&,.!?(){}\[\]]|==|<=|&&|\|\|").unwrap().is_match("1a11"));
        // println!("括号匹配 {}",Regex::new(r"[A-Z_a-z][A-Z_a-z0-9]*|[+\-*/%=\\|&,.!?(){}\[\]]|==|<=|&&|\|\|").unwrap().is_match("====="));
        // println!("括号匹配 {}",Regex::new(r"^([A-Z_a-z][A-Z_a-z0-9]*|[+\-*/%=\\|&,.!?(){}\[\]]|==|<=|&&|\|\|)").unwrap().is_match("1a11"));
        // println!("括号匹配 {}",Regex::new(r"^([A-Z_a-z][A-Z_a-z0-9]*|[+\-*/%=\\|&,.!?(){}\[\]]|==|<=|&&|\|\|)").unwrap().is_match("====="));
        println!("字符串 {}",Regex::new(r"^([A-Z_a-z][A-Z_a-z0-9]*)").unwrap().is_match("1a11"));
        println!("字符串 {}",Regex::new(r"^([A-Z_a-z][A-Z_a-z0-9]*)").unwrap().is_match("a11"));
        println!("字符串 {}",Regex::new(r"^([A-Z_a-z][A-Z_a-z0-9]*)").unwrap().is_match("_a11"));
        println!("字符串 {}",Regex::new(r"^([A-Z_a-z][A-Z_a-z0-9]*)").unwrap().is_match("_1a1aa"));

        println!("符号 {}",Regex::new(r"[+\-*/%=\\|&,.!?(){}\[\]]").unwrap().is_match("====="));
        println!("符号 {}",Regex::new(r"[=+]").unwrap().is_match("+++++"));

        //
        // let re = Regex::new(r"^([A-Z_a-z][A-Z_a-z0-9]*)").unwrap();
        // println!("Match result: {}",  re.is_match("1a11"));
        // println!("Match result: {}",  re.is_match("a11"));
    }

    #[test]
    fn token_identifier_match_test() {
        // let test_token_legal_vec = vec![
        //     "+",
        //     "-",
        //     "*",
        //     "/",
        //     "%",
        //     "=",
        //     "\\",
        //     "|",
        //     "&",
        //     ",",
        //     ".",
        //     "!",
        //     "?",
        //     "(",
        //     ")",
        //     "[",
        //     "]",
        //     "{",
        //     "}",
        //     "||", "a111", "a_11111", "==", "<=", "&&"];
        // for test_token in test_token_legal_vec {
        //     let token_match = TokenIdentifier::match_token(test_token);
        //     assert_eq!(token_match, true, "token {} is not match but should match", test_token);
        // }
        // println!("[TokenIdentifier]:legal simple test pass");
        //
        // let test_token_illegal_vec = vec!["1a11", "【【", "[[", "]]"];
        // for test_token in test_token_illegal_vec {
        //     let token_match = TokenIdentifier::match_token(test_token);
        //     assert_ne!(token_match,true, "token {} should not match but match", test_token);
        // }
        // println!("[TokenIdentifier]:illegal simple test pass");
    }


    // #[test]
    // fn token_identifier_new_test() {
    //     let identifier = TokenIdentifier::new(1, "=".to_string());
    //     let x = identifier.line_number();
    //     println!("identifier {:#?}", identifier);
    //     println!("line number {}", x);
    // }
    //
    // #[test]
    // #[should_panic]
    // fn token_identifier_new_panic_test() {
    //     let identifier = TokenIdentifier::new(0, "=".to_string());
    //     println!("identifier {:#?}",identifier);
    // }
    //
    //
    // #[test]
    // #[should_panic]
    // fn token_identifier_new_panic_test_2() {
    //     let identifier = TokenIdentifier::new(1, "【".to_string());
    //     println!("identifier {:#?}",identifier);
    // }


    #[test]
    fn line_reader_regex_test() {
        // let token_str = TokenStr {
        //     line_number: 1,
        //     value: "ddddd".to_string()
        // };
        //
        // let vec:Vec<dyn Token> = vec![];
    }

    # [test]
    fn token_eq_test() {
        let identifier_1 = TokenValue::IDENTIFIER("abc".to_string());
        let identifier_2 = TokenValue::IDENTIFIER("abc".to_string());
        let identifier_3 = TokenValue::IDENTIFIER("abcd".to_string());
        let identifier_4 = TokenValue::StringVal("abcd".to_string());

        println!("eq? {}", (identifier_1 == identifier_2));
        println!("eq? {}", (identifier_1 == identifier_3));
        println!("eq? {}", (identifier_3 == identifier_4));
    }
}