use crate::ast::list::binary_expr::BinaryExprFactory;
use crate::parser::element::Operators;
use crate::parser::parser::Parser;
use crate::token::TokenValue;
use crate::{expr, op, or, rule, seq};
use std::rc::Rc;

pub fn stone_parser_with_func() -> Parser {
    let reserved = vec!["}", ";", TokenValue::literal_eol(),")"];

    let op: Rc<Operators> = op! {
        right 1 =;
        left  2 ==,>,<;
        left  3 +,-;
        left  4 *,/,% ;
    };

    let mut expr = rule!();
    let mut statement = rule!();
    let mut block = seq!(block: );

    // 函数定义语法
    let param = seq!(seq: id->reserved);
    let params = seq!(param_list: param { "," param });
    // 注意这里展开后使用的是 maybe
    let param_list = seq!(seq: "(" [ params ]* ")");
    let def = seq!(def: "def" id->reserved param_list block );

    let args = seq!(args:  expr { "," expr });
    // 这里的可选参数也应该是maybe
    let postfix = seq!(seq: "(" [args]* ")");


    let primary =
        or!(primary:seq!{fun: "fun" param_list block } , seq!{seq: "(" expr ")"},  seq!(seq: number),  seq!(seq: id->reserved), seq!(seq:string));
    seq!(primary; seq : {postfix});

    let factor = or!(seq!(neg: "-" primary),primary);

    expr!(expr: factor { op factor });

    seq!(block; seq : "{" [ statement ]  { (";","\n") [ statement ] } "}" );

    let simple = seq!(primary:expr [ args ]);

    or!(statement;
        seq!(if : "if" expr block [ "else" block ]),
        seq!(while: "while" expr block),
        simple
    );

    or!(no_rc: def  , statement, seq!(null:(";","\n")) )
}