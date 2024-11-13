#[cfg(test)]
mod line_reader_lexer_text {
    use crate::lexer::line_reader_lexer::{*};
    use regex::{Match, Regex};

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
    pub fn mul_line_match_test() {
        let test_code = r#"
           i=j>0 || j = 0 ? "中文111111":"英文"
        "#;
        let mut lexer = LineReaderLexer::new();
        lexer.read_line(test_code, 1);
    }
}