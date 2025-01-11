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
    ( $leaf_name:ident, $factory:expr $(,$param:ident)* ) => {
        crate::parser::parser::Parser::rule_def().$leaf_name(Some( $factory ), $($param,)* ).rc()
    }
}

#[macro_export]
macro_rules! number {
    () => {
         leaf!(number,NumberLiteralFactory::new())
    }
}

#[macro_export]
macro_rules! identifier {
    ($reserved:ident) => {
         leaf!(identifier,IdentifierLiteralFactory::new(),$reserved )
    }
}

#[macro_export]
macro_rules! string {
    () => {
         leaf!(string,StringLiteralFactory::new())
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
        crate::parser::parser::Parser::rule($factory).or( vec![ $(&$or,)* ] ).rc()
    };

    (no_rc: $($or:expr),*) => {
        crate::parser::parser::Parser::rule_def().or( vec![ $(&$or,)* ])
    };

    ($($or:expr),*) => {
        crate::parser::parser::Parser::rule_def().or( vec![ $(&$or,)* ] ).rc()
    };
}

#[macro_export]
macro_rules! or_primary {
   ($($or:expr),*) => {
        or!(factory crate::ast::list::primary_expr::PrimaryExprFactory::new(); $($or),*)
    };
}

#[macro_export]
macro_rules! or_for {
   ($obj:ident; $($or:expr),*) => {
       $obj.borrow_mut().or_ref( vec![ $(&$or,)* ] );
   };
}


#[macro_export]
macro_rules! seq {
    () => {};

    (seq: $($tail:tt) *) => {
       {
           let mut seq = crate::parser::parser::Parser::rule_def();
           seq!(seq -> $($tail)*);
           seq.rc()
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

    (factory $factory:expr;seq : $($tail:tt) *) => {
       {
           let mut seq = crate::parser::parser::Parser::rule($factory);
           seq!(seq -> $($tail)*);
           seq.rc()
       }
    };

    ($seq:ident ->) => {};

    ($seq:ident -> ($($lt:literal),+) $($tail:tt)*) => {
        $seq = $seq.sep(vec![ $($lt),+ ]);
        seq!($seq -> $($tail)*);
    };

    ($seq:ident -> $lt:literal $($tail:tt)*) => {
        $seq = $seq.sep(vec![$lt]);
        seq!($seq -> $($tail)*);
    };

    ($seq:ident -> $ident:ident $($tail:tt)*) => {
        $seq = $seq.ast(&$ident);
        seq!($seq -> $($tail)*);
    };

    ($seq:ident -> [$op:ident] $($tail:tt)*) => {
        $seq = $seq.option(&$op);
        seq!($seq -> $($tail)*);
    };

    ($seq:ident -> [$($seq_op:tt)*] $($tail:tt)*) => {
        $seq = $seq.option(&seq!($seq: $($seq_op)*));
        seq!($seq -> $($tail)*);
    };

    ($seq:ident -> {$repeat:ident} $($tail:tt)*) => {
        $seq = $seq.repeat($repeat);
        seq!($seq -> $($tail)*);
    };

    ($seq:ident -> {$($seq_repeat:tt)*} $($tail:tt)*) => {
        $seq = $seq.repeat(&seq!($seq: $($seq_repeat)*));
        seq!($seq -> $($tail)*);
    };
}

#[macro_export]
macro_rules! expr {
    ($expr:ident: $factor:ident { $op:ident $factor_dup:ident }) => {
        $expr.borrow_mut().expr_ref(BinaryExprFactory::new(), &$factor, &$op);
    };
}
