use std::any::TypeId;
use std::ops::DerefMut;
use crate::ast::ast_list::AstList;
use crate::ast::ast_tree::AstTree;
use crate::ast::list::null_stmt::NullStmt;
use crate::eval::environment::EnvWrapper;
use crate::lexer::lexer::Lexer;
use crate::lexer::line_reader_lexer::LineReaderLexer;
use crate::parser::func_parser::stone_parser_with_func;
use crate::parser::parser::Parser;
use crate::token::TokenValue;

pub struct StoneLang<'s> {
    lexer: Box<dyn Lexer + 's>,
    parser: Parser,
}

impl<'s> StoneLang<'s> {
    pub fn new_def() -> StoneLang<'s> {
        StoneLang::<'s> {
            lexer: Box::new(LineReaderLexer::new()),
            parser: stone_parser_with_func(),
        }
    }

    pub fn code_2_ast(&mut self, code: String) -> Result<Vec<Box<dyn AstTree>>, String> {
        let mut lexer = &mut self.lexer;
        let mut parser = &mut self.parser;

        let mut ast_tree_res_vec: Vec<Box<dyn AstTree>> = vec![];

        lexer.reset_code(code);
        while let Some(token) = lexer.peek(0) {
            if TokenValue::EOF.eq(token.value()){
                break;
            }
            if !parser.is_match(lexer.deref_mut()) {
                return Err(format!("无法处理的token： {:?}", lexer.read().unwrap().value()));
            }
            let parser_res = parser.parse(lexer.deref_mut())?;
            if parser_res.actual_type_id() == TypeId::of::<NullStmt>() { continue; }
            ast_tree_res_vec.push(parser_res);
        }
        Ok(ast_tree_res_vec)
    }

    pub fn code_eval(&mut self, code: String) -> Result<EnvWrapper, String> {
        let mut env = EnvWrapper::new();

        let ast_tree_res_vec = self.code_2_ast(code)?;
        for (index,ast_tree_one) in ast_tree_res_vec.iter().enumerate() {
            let res = ast_tree_one.eval().do_eval(&mut env)?;
            println!("eval res: {:?}",res);
        }
        Ok(env)
    }
}