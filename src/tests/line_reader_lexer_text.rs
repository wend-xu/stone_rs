#[cfg(test)]
mod line_reader_lexer_text {
    use crate::lexer::line_reader_lexer::{*};
    use regex::{Match, Regex};


    #[test]
    pub fn test_matcher_string(){
        println!("{:?}", MATCH_STRING);
        let regex = Regex::new(MATCH_STRING).unwrap();
        let str_vec = vec![
            ("hello world", false),
            ("\"hello world\"",true),
            ("\"hello \\\" world\"",true)
        ];

        for (str,result) in str_vec {
            let should_match = regex.is_match(&str);
            println!("{} : {}", str, should_match);
            assert_eq!(should_match, result);
        }
    }


    #[test]
    pub fn test_matcher_number(){
        println!("{:?}",MATCH_NUMBER);
        let regex = Regex::new(MATCH_NUMBER).unwrap();
        let str_vec = vec![
            ("hello world", false),
            ("99",true),
            ("99999",true),
        ];

        for (str,result) in str_vec {
            let should_match = regex.is_match(&str);
            println!("{} : {}", str, should_match);
            assert_eq!(should_match, result);
        }
    }

    #[test]
    pub fn test_matcher_identifier(){
        println!("{:?}",MATCH_IDENTIFIER);
        let regex = Regex::new(MATCH_IDENTIFIER).unwrap();
        let identifier_vec = vec![
            ("+",true),
            ("-",true),
            ("*",true),
            ("/",true),
            ("%",true),
            ("=",true),
            ("\\",true),
            ("|",true),
            ("&",true),
            (",",true),
            (".",true),
            ("!",true),
            ("?",true),
            ("(",true),
            (")",true),
            ("[",true),
            ("]",true),
            ("{",true),
            ("}",true),
            ("||",true),
            ("a111",true),
            ("a_11111",true),
            ("==",true),
            ("<=",true),
            ("&&",true),
            // ("【【",false),
            ("[[",true),
            ("]]",true)
        ];

        for (identifier,result) in identifier_vec {
            let should_match = regex.is_match(&identifier);
            println!("{} : {}", identifier, should_match);
            assert_eq!(should_match, result);
        }
    }

    #[test]
    pub fn line_match_test(){
        let test_code = "i=j>0 || j = 0 ? \"中文111111\"";
        let regex = Regex::new(match_line_regex_str().as_str()).unwrap();

        for sub_cap in regex.captures_iter(test_code) {

        }
        // if regex.is_match_at(test_code,0) {
        //     let find = regex.captures_at(test_code, 0).unwrap();
        //     println!("0 : {}",find.name("comment").map_or("", |m| m.as_str()));
        //     println!("1 : {}",find.name("number").map_or("", |m| m.as_str()));
        //     println!("2 : {}",find.name("string").map_or("", |m| m.as_str()));
        //     println!("3 : {}",find.name("identifier").map_or("", |m| m.as_str()));
        // }
    }


    #[test]
    pub fn line_match_test_2(){
        let test_code = "i=j>0 || j = 0 ? \"中文111111\"";
        let max = test_code.len();

        let match_line = match_line_regex_str();
        let regex = Regex::new(match_line.as_str()).unwrap();

        // let regex = Regex::new( r#"[A-Z_a-z][A-Z_a-z0-9]*|==|<=|>=|&&|\|\||[=+]"#).unwrap();
        // let regex = Regex::new(MATCH_IDENTIFIER).unwrap();
        // println!("{:?}", match_line);
        // let mut i:i32 = 0;
        // while i < test_code.len() as i32 && i != -1{
        //     i = match_and_println(&regex,test_code,i);
        // }
        // match_and_println_1(&regex,test_code,0);
        // match_and_println_1(&regex,test_code,1);
        // match_and_println_1(&regex,test_code,2);
        // match_and_println_1(&regex,test_code,3);
        // match_and_println_1(&regex,test_code,4);
        // match_and_println_1(&regex,test_code,6);

        // let test_code = "i=i+2*3";
        let lexer = LineReaderLexer::new();
        lexer.read_line(test_code,1);
    }


    fn match_and_println(regex:&Regex,str:&str,start:i32) ->i32{
        if regex.is_match(str) {
            match  regex.find_at(str, start as usize) {
                None => {
                    -1
                }
                Some(mat) => {
                    println!("开始位置 {start} 匹配结果:{} , 匹配终点：{}",mat.as_str(),mat.end());
                    mat.end() as i32
                }
            }
        }else{
            println!("开始位置 {start} 未匹配到结果");
            -1
        }
    }

}