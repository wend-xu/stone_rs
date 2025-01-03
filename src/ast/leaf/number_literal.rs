use crate::ast::ast_leaf::AstLeaf;
use crate::{ast_impl_leaf_factory, ast_leaf_impl_for, ast_leaf_new_for};
use crate::eval::environment::EnvWrapper;
use crate::eval::eval::{EvalRes, Evaluate};
use crate::token::TokenValue;

pub struct NumberLiteral {
    ast_leaf: AstLeaf,
}

impl NumberLiteral {
    ast_leaf_new_for! {NumberLiteral,NUMBER }
}

ast_leaf_impl_for! {NumberLiteral,TokenNumber}

ast_impl_leaf_factory! {NumberLiteralFactory,NumberLiteral}

impl Evaluate for NumberLiteral {
    fn do_eval(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        let token = self.leaf_val();
        let eval_res = match token {
            TokenValue::NUMBER(id) => {
                EvalRes::NUMBER(id.clone())
            }
            _ => {
                panic!("[NumberLiteral] hold token must a TokenValue::NUMBER , not match \n error may occur in build AstTree")
            }
        };
        Ok(eval_res)
    }
}