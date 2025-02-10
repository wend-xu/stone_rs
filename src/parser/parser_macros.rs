#[macro_export]
macro_rules! op {
    ($($assoc:tt $priority:tt $($op:tt),+ ;)+) => {
        {
            let mut operators = crate::parser::element::Operators::new();
            $(
                $(
                    operators.add(crate::parser::element::Precedence::$assoc(stringify!($op), $priority));
                )+
            )+
            operators.rc()
        }
    };
}

#[macro_export]
macro_rules! leaf {
    ( $leaf_name:ident, $factory:expr $(,$param:expr)* ) => {
        crate::parser::parser::Parser::rule_def().$leaf_name(Some( $factory ), $($param,)* ).rc()
    }
}


#[macro_export]
macro_rules! rule {
    () => {
        crate::parser::parser::Parser::rule_def().rc()
    };
}

#[macro_export]
macro_rules! or {
    (factory $factory:expr; $($or:expr),*) => {
        crate::parser::parser::Parser::rule($factory).or_owner( vec![ $(&$or,)* ] ).rc()
    };

    (no_rc: $($or:expr),*) => {
        crate::parser::parser::Parser::rule_def().or_owner( vec![ $(&$or,)* ])
    };

    (primary: $($or:expr),*) => {
        or!(factory crate::ast::list::primary_expr::PrimaryExprFactory::new(); $($or),*)
    };

    ($obj:ident; $($or:expr),*) => {
       $obj.borrow_mut().or( vec![ $(&$or,)* ] );
    };
    
    ($obj:ident -> $($or:expr),*) => {
       $obj.or( vec![ $(&$or,)* ] );
    }; 

    ($($or:expr),*) => {
        crate::parser::parser::Parser::rule_def().or_owner( vec![ $(&$or,)* ] ).rc()
    };
}

#[macro_export]
macro_rules! seq {
    () => {};

    (seq: $($tail:tt) *) => {
       {
           let mut parser = crate::parser::parser::Parser::rule_def().rc();
           {
               let mut seq = parser.borrow_mut();
               seq!(seq -> $($tail)*);
           }
           parser
       }
    };

    (if: $($tail:tt) *) => {
       seq!(factory crate::ast::list::if_stmt::IfStmtFactory::new(); seq : $($tail) *)
    };

    (while: $($tail:tt) *) => {
       seq!(factory crate::ast::list::while_stmt::WhileStmtFactory::new(); seq : $($tail) *)
    };

    (neg: $($tail:tt) *) => {
       seq!(factory crate::ast::list::while_stmt::WhileStmtFactory::new(); seq : $($tail) *)
    };

    (block: $($tail:tt) *) => {
       seq!(factory crate::ast::list::block_stmt::BlockStmtFactory::new(); seq : $($tail) *)
    };

    (primary: $($tail:tt) *) => {
       seq!(factory crate::ast::list::primary_expr::PrimaryExprFactory::new(); seq : $($tail) *)
    };

    (null: $($tail:tt) *) => {
       seq!(factory crate::ast::list::null_stmt::NullStmtFactory::new(); seq : $($tail) *)
    };

    (param_list: $($tail:tt) *) => {
       seq!(factory crate::ast::list::paramter_list::ParameterListFactory::new(); seq : $($tail) *)
    };

    (args: $($tail:tt) *) => {
       seq!(factory crate::ast::list::arguments::ArgumentsFactory::new(); seq : $($tail) *)
    };

    (def: $($tail:tt) *) => {
       seq!(factory crate::ast::list::def_stmt::DefStmtFactory::new(); seq : $($tail) *)
    };


    (factory $factory:expr;seq : $($tail:tt) *) => {
       {
           let parser = crate::parser::parser::Parser::rule($factory).rc();
           {
               let mut seq = parser.borrow_mut();
               seq!(seq -> $($tail)*);
           }
           parser
       }
    };

    ($obj:ident; seq : $($tail:tt) *) => {
        {
           let mut seq = $obj.borrow_mut();
           seq!(seq -> $($tail)*);
        }
    };

    ($seq:ident ->) => {
    };

    ($seq:ident -> id->$reserved:ident $($tail:tt)*) => {
        $seq.identifier(Some(crate::ast::leaf::identifier_literal::IdentifierLiteralFactory::new()), &$reserved);
        seq!($seq -> $($tail)*);
    };

    ($seq:ident -> string $($tail:tt)*) => {
        $seq.string(Some(crate::ast::leaf::string_literal::StringLiteralFactory::new()));
        seq!($seq -> $($tail)*);
    };

    ($seq:ident -> number $($tail:tt)*) => {
        $seq.number(Some(crate::ast::leaf::number_literal::NumberLiteralFactory::new()));
        seq!($seq -> $($tail)*);
    };

    ($seq:ident -> ($($lt:literal),+) $($tail:tt)*) => {
        $seq.sep(vec![ $($lt),+ ]);
        seq!($seq -> $($tail)*);
    };

    ($seq:ident -> $lt:literal $($tail:tt)*) => {
        $seq.sep(vec![$lt]);
        seq!($seq -> $($tail)*);
    };

    ($seq:ident -> $ident:ident $($tail:tt)*) => {
        $seq.ast(&$ident);
        seq!($seq -> $($tail)*);
    };

    ($seq:ident -> [$op:ident]+ $($tail:tt)*) => {
        $seq.maybe(&$op);
        seq!($seq -> $($tail)*);
    };

    ($seq:ident -> [$op:ident] $($tail:tt)*) => {
        $seq.option(&$op);
        seq!($seq -> $($tail)*);
    };

    ($seq:ident -> [$($seq_op:tt)*] $($tail:tt)*) => {
        $seq.option(&seq!($seq: $($seq_op)*));
        seq!($seq -> $($tail)*);
    };

    ($seq:ident -> {$repeat:ident} $($tail:tt)*) => {
        $seq.repeat(&$repeat);
        seq!($seq -> $($tail)*);
    };

    ($seq:ident -> {$($seq_repeat:tt)*} $($tail:tt)*) => {
        $seq.repeat(&seq!($seq: $($seq_repeat)*));
        seq!($seq -> $($tail)*);
    };
}

#[macro_export]
macro_rules! expr {
    ($expr:ident: $factor:ident { $op:ident $factor_dup:ident }) => {
        $expr.borrow_mut().expr_ref(BinaryExprFactory::new(), &$factor, &$op);
    };
}
