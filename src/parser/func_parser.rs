use crate::ast::leaf::identifier_literal::IdentifierLiteralFactory;
use crate::ast::leaf::number_literal::NumberLiteralFactory;
use crate::ast::leaf::string_literal::StringLiteralFactory;
use crate::ast::list::binary_expr::BinaryExprFactory;
use crate::parser::element::Operators;
use crate::parser::parser::Parser;
use crate::token::TokenValue;
use crate::{expr, identifier, leaf, number, op, or, rule, seq, string};
use std::rc::Rc;

pub fn stone_parser_with_func() -> Parser {
    let reserved = vec!["}", ";", TokenValue::literal_eol()];

    let op: Rc<Operators> = op! {
        right 1 =;
        left  2 ==,>,<;
        left  3 +,-;
        left  4 *,/,% ;
    };
    // 函数定义语法
    let identifier = identifier!(&reserved);
    let param = identifier!(&reserved);
    let params = seq!(seq: param { "," param });
    // 注意这里展开后使用的是 maybe
    let param_list = seq!(seq: "(" [ params ] ")");
    // let def = seq!(seq: "def" identifier!(&reserved) );


    let mut expr = rule!();
    let mut statement = rule!();



    let primary =
        or!(primary: seq!{seq: "(" expr ")"}, number!(), identifier!(&reserved) , string!());

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