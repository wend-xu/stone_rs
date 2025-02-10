use crate::ast::list::binary_expr::BinaryExprFactory;
use crate::parser::element::Operators;
use crate::parser::parser::Parser;
use crate::token::TokenValue;
use crate::{expr, op, or, rule, seq};
use std::rc::Rc;
use crate::ast::list::primary_expr::PrimaryExprFactory;

pub fn stone_parser() -> Parser {
    let reserved = vec!["}", ";", TokenValue::literal_eol()];

    let op: Rc<Operators> = op! {
        right 1 =;
        left  2 ==,>,<;
        left  3 +,-;
        left  4 *,/,% ;
    };
    let mut expr = rule!();
    let mut statement = rule!();

    // primary   :  "(" expr ") " | NUMBER | IDENTIFIER | STRING
    // factor    :  "-" primary | primary
    // expr      :  factor { Op factor }
    // block     :  "{" [ statement ] {(";" | EOL) [ statement ]} "}"
    // simple    :  expr
    // statement :  "if" expr block [ "else" block ]
    // 			  |"while" expr block
    // 			  |simple
    // program   :  [ statement ] (";" | EOL)
    let primary =
        or!(primary: seq!{seq: "(" expr ")"},  seq!(seq: number),  seq!(seq: id->reserved), seq!(seq:string));

    or!(primary: seq!{seq: "(" expr ")"} );

    let factor = or!(seq!(neg: "-" primary),primary);

    expr!(expr: factor { op factor });

    let block =
        seq!(block: "{" [ statement ]  { (";","\n") [ statement ] } "}" );

    let simple = seq!(primary:expr);

    or!(statement;
        seq!(if : "if" expr block [ "else" block ]),
        seq!(while: "while" expr block),
        simple
    );

    or!(no_rc: statement, seq!(null:(";","\n")) )
}
