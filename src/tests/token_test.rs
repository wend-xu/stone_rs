#[cfg(test)]
mod token_tests {
    use regex::Regex;
    use crate::token::Token;
    // use crate::token::token_end::TokenEOL;
    use crate::token::token_identifier::TokenIdentifier;

    use crate::lexer::line_reader_lexer;


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

    }
}