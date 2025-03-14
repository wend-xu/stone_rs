use crate::ast::list::binary_expr::BinaryExprFactory;
use crate::ast::list::block_stmt::BlockStmtFactory;
use crate::ast::leaf::identifier_literal::IdentifierLiteralFactory;
use crate::ast::list::if_stmt::IfStmtFactory;
use crate::ast::list::negative_expr::NegativeExprFactory;
use crate::ast::list::null_stmt::NullStmtFactory;
use crate::ast::leaf::number_literal::NumberLiteralFactory;
use crate::ast::list::primary_expr::PrimaryExprFactory;
use crate::ast::leaf::string_literal::StringLiteralFactory;
use crate::ast::list::while_stmt::WhileStmtFactory;
use crate::parser::element::{Operators, Precedence};
use crate::parser::parser::Parser;
use crate::token::TokenValue;

pub fn stone_parser() -> Parser {
    // let reserved = vec!["}", ";", TokenValue::literal_eol()];
    //
    // let mut operators = Operators::new();
    // operators.add(Precedence::right("=", 1));
    // operators.add(Precedence::left("==", 2));
    // operators.add(Precedence::left(">", 2));
    // operators.add(Precedence::left("<", 2));
    // operators.add(Precedence::left("+", 3));
    // operators.add(Precedence::left("-", 3));
    // operators.add(Precedence::left("*", 4));
    // operators.add(Precedence::left("/", 4));
    // operators.add(Precedence::left("%", 4));
    // let operators = operators.rc();
    //
    // let mut expr = Parser::rule_def().rc();
    //
    //
    // let primary = Parser::rule(PrimaryExprFactory::new())
    //     .or(vec![
    //         &Parser::rule_def().sep(vec!["("]).ast(&expr).sep(vec![")"]).rc(),
    //         &Parser::rule_def().number(Some(NumberLiteralFactory::new())).rc(),
    //         &Parser::rule_def().identifier(Some(IdentifierLiteralFactory::new()), &reserved).rc(),
    //         &Parser::rule_def().string(Some(StringLiteralFactory::new())).rc()
    //     ]).rc();
    //
    // let factor = Parser::rule_def()
    //     .or(vec![
    //         &Parser::rule(NegativeExprFactory::new()).sep(vec!["-"]).ast(&primary).rc(),
    //         &primary
    //     ]).rc();
    //
    // expr.borrow_mut().expr_ref(BinaryExprFactory::new(), &factor, &operators);
    //
    // let mut statement = Parser::rule_def().rc();
    //
    // let block = Parser::rule(BlockStmtFactory::new())
    //     .sep(vec!["{"])
    //     .option(&statement)
    //     .repeat(&Parser::rule_def().sep(vec![";", TokenValue::literal_eol()]).option(&statement).rc())
    //     .sep(vec!["}"])
    //     .rc();
    //
    // let simple = Parser::rule(PrimaryExprFactory::new()).ast(&expr).rc();
    //
    // statement.borrow_mut().or_ref(vec![
    //     &Parser::rule(IfStmtFactory::new())
    //         .sep(vec!["if"]).ast(&expr)
    //         .ast(&block)
    //         .option(&Parser::rule_def().sep(vec!["else"]).ast(&block).rc()).rc(),
    //     &Parser::rule(WhileStmtFactory::new()).sep(vec!["while"]).ast(&expr).ast(&block).rc(),
    //     &simple
    // ]);
    //
    // Parser::rule_def().or(vec![&statement, &Parser::rule(NullStmtFactory::new()).sep(vec![";", TokenValue::literal_eol()]).rc()])
    todo!()
}