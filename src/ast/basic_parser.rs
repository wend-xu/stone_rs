use crate::ast::element::{Operators, Precedence};
use crate::ast::factory::{BinaryExprFactory, BlockStmtFactory, IdentifierLiteralFactory, IfStmtFactory, NegativeExprFactory, NullStmtFactory, NumberLiteralFactory, PrimaryExprFactory, StringLiteralFactory, WhileStmtFactory};
use crate::ast::parser::Parser;

pub fn stone_parser() -> Parser {
    let mut operators = Operators::new();
    operators.add("=", Precedence::right(1));
    operators.add("==", Precedence::left(2));
    operators.add(">", Precedence::left(2));
    operators.add("<", Precedence::left(2));
    operators.add("+", Precedence::left(3));
    operators.add("-", Precedence::left(3));
    operators.add("*", Precedence::left(4));
    operators.add("/", Precedence::left(4));
    operators.add("%", Precedence::left(4));
    let operators = operators.rc();

    let mut expr = Parser::rule_def().rc();

    let primary = Parser::rule(PrimaryExprFactory::new())
        .or(vec![
            &Parser::rule_def().sep(vec!["("]).ast(&expr).sep(vec![")"]).rc(),
            &Parser::rule_def().number(Some(NumberLiteralFactory::new())).rc(),
            &Parser::rule_def().identifier(Some(IdentifierLiteralFactory::new())).rc(),
            &Parser::rule_def().string(Some(StringLiteralFactory::new())).rc()
        ]).rc();

    let factor = Parser::rule_def()
        .or(vec![
            &Parser::rule(NegativeExprFactory::new()).sep(vec!["-"]).ast(&primary).rc(),
            &primary
        ]).rc();

    expr.expr_ref(BinaryExprFactory::new(), &factor, &operators);

    let mut statement = Parser::rule_def().rc();

    let block = Parser::rule(BlockStmtFactory::new())
        .sep(vec!["{"])
        .option(&statement).repeat(
            &Parser::rule_def().sep(vec![";", "\\n"]).option(&statement).rc()
        )
        .sep(vec!["}"]).rc();

    let simple = Parser::rule(PrimaryExprFactory::new()).ast(&expr).rc();

    statement.or_ref(vec![
        &Parser::rule(IfStmtFactory::new())
            .sep(vec!["if"]).ast(&expr)
            .ast(&block)
            .option(&Parser::rule_def().sep(vec!["else"]).ast(&block).rc()).rc(),
        &Parser::rule(WhileStmtFactory::new()).sep(vec!["while"]).ast(&expr).ast(&block).rc(),
        &simple
    ]);

    Parser::rule_def().or(vec![&statement, &Parser::rule(NullStmtFactory::new()).rc()]).sep(vec![";", "\n"])
}