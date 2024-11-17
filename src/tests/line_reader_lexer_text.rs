#[cfg(test)]
mod line_reader_lexer_text {
    use crate::lexer::lexer::Lexer;
    use crate::lexer::line_reader_lexer::*;
    use regex::Regex;
    use std::time::SystemTime;
    use crate::token::Token;

    #[test]
    pub fn test_matcher_string() {
        println!("{:?}", MATCH_STRING);
        let regex = Regex::new(MATCH_STRING).unwrap();
        let str_vec = vec![
            ("hello world", false),
            ("\"hello world\"", true),
            ("\"hello \\\" world\"", true)
        ];

        for (str, result) in str_vec {
            let should_match = regex.is_match(&str);
            println!("{} : {}", str, should_match);
            assert_eq!(should_match, result);
        }

        let str = r#""hello world":"fuck the world""#;
        let find0 = regex.find(&str).unwrap();
        println!("{} - {}", find0.as_str(), find0.end());

        let find1 = regex.find_at(&str, 13).unwrap();
        println!("{} - {}", find1.as_str(), find1.end());
    }


    #[test]
    pub fn test_matcher_number() {
        println!("{:?}", MATCH_NUMBER);
        let regex = Regex::new(MATCH_NUMBER).unwrap();
        let str_vec = vec![
            ("hello world", false),
            ("99", true),
            ("99999", true),
        ];

        for (str, result) in str_vec {
            let should_match = regex.is_match(&str);
            println!("{} : {}", str, should_match);
            assert_eq!(should_match, result);
        }
    }

    #[test]
    pub fn test_matcher_identifier() {
        println!("{:?}", MATCH_IDENTIFIER);
        let regex = Regex::new(MATCH_IDENTIFIER).unwrap();
        let identifier_vec = vec![
            ("+", true),
            (":", true),
            ("-", true),
            ("*", true),
            ("/", true),
            ("%", true),
            ("=", true),
            ("\\", true),
            ("|", true),
            ("&", true),
            (",", true),
            (".", true),
            ("!", true),
            ("?", true),
            ("(", true),
            (")", true),
            ("[", true),
            ("]", true),
            ("{", true),
            ("}", true),
            ("||", true),
            ("a111", true),
            ("a_11111", true),
            ("==", true),
            ("<=", true),
            ("&&", true),
            // ("【【",false),
            ("[[", true),
            ("]]", true)
        ];

        for (identifier, result) in identifier_vec {
            let should_match = regex.is_match(&identifier);
            println!("{} : {}", identifier, should_match);
            assert_eq!(should_match, result);
        }
    }

    #[test]
    pub fn line_match_test() {
        let test_code = "i=j>0 || j = 0 ? \"中文111111\"";
        let mut lexer = LineReaderLexer::new(test_code.to_string());
        // lexer.read_line(test_code, 1);
        lexer.read_line();
    }

    #[test]
    pub fn mul_line_match_comment_test() {
        let test_code = r#"
           // size = j.len()
        "#;
        let mut lexer = LineReaderLexer::new(test_code.to_string());
        while let Some(token_box) = lexer.read() {
            println!("{:?}", token_box.value());
        }
        // println!("{}", lexer);
    }

    #[test]
    pub fn mul_line_match_test() {
        let test_code = r#"
           i=j>0 || j = 0 ? "中文111111":"中文1222222"
            size = j.len()
        "#;
        let mut lexer = LineReaderLexer::new(test_code.to_string());
        while let Some(token_box) = lexer.read() {
            println!("{:?}", token_box.value());
        }
    }


    #[test]
    pub fn line_match_test_10000() {
        let test_code = r#"
           i=j>0 || j = 0 ? "中文111111":"中文1222222"
            size = j.len()
        "#;
        let mut lexer = LineReaderLexer::new(test_code.to_string());
        while let Some(token_box) = lexer.read() {
            println!("{:?}", token_box.value());
        }
        let match_line = match_line_regex_str();
        let match_line_regex: Regex = Regex::new(match_line.as_str()).unwrap();

        let start = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
        for i in 0..10000 {
            let lexer = LineReaderLexer::new_with_regex(test_code.to_string(), match_line_regex.clone());
        }
        let end = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
        println!("耗时 {} ", end - start);
    }


    #[test]
    pub fn mul_line_peek_test() {
        let test_code = r#"
           i=j>0 || j = 0 ? "中文111111":"中文1222222"
            size = j.len()
        "#;
        let mut lexer = LineReaderLexer::new(test_code.to_string());
        // while let Some(token_box) = lexer.read() {
        //     println!("{:?}", token_box.value());
        // }
        println!(" 重复读取测试： ");
        for i in 0.. 10 {
            if let Some(token_box) = lexer.peek(1) {
                println!("{:?}", token_box.value());
            }
        }

        println!(" 顺序预读取测试： ");
        for i in 0.. 30 {
            if let Some(token_box) = lexer.peek(i) {
                println!("{:?}", token_box.value());
            }else{
                println!("没有更多 token ...")
            }
        }
    }
}