#[cfg(test)]
mod line_reader_lexer_text {
    use std::time::SystemTime;
    use crate::lexer::line_reader_lexer::{*};
    use regex::{Match, Regex};
    use crate::lexer::lexer::Lexer;

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
        println!("{} - {}",find0.as_str() , find0.end());

        let find1 = regex.find_at(&str,13).unwrap();
        println!("{} - {}",find1.as_str() , find1.end());
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
        let mut lexer = LineReaderLexer::new();
        lexer.read_line(test_code, 1);
    }

    #[test]
    pub fn mul_line_match_comment_test() {
        let test_code = r#"
           // size = j.len()
        "#;
        let mut lexer = LineReaderLexer::new();
        lexer.read(test_code.to_string());
        println!("{}", lexer);
    }

    #[test]
    pub fn mul_line_match_test() {
        let test_code = r#"
           i=j>0 || j = 0 ? "中文111111":"中文1222222"
            size = j.len()
        "#;
        let mut lexer = LineReaderLexer::new();
        lexer.read(test_code.to_string());
        println!("{}", lexer);
    }


    #[test]
    pub fn line_match_test_10000() {
        let test_code = r#"i=j>0 || j = 0 ? "中文111111":"中文1222222""#;
        let mut lexer = LineReaderLexer::new();
        lexer.read(test_code.to_string());
        test_code.to_string();
        println!("{}", lexer);
        let start = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
        for i in 0..10000 {
            let mut lexer = LineReaderLexer::new();
            lexer.read(test_code.to_string());
        }
        let end = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
        println!("耗时 {} ", end - start);
    }

}