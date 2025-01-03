use crate::ast::ast_leaf::AstLeaf;
use crate::{ast_leaf_factory_default_impl, ast_leaf_default_impl, ast_leaf_default_new};
use crate::eval::environment::EnvWrapper;
use crate::eval::eval::{EvalRes, Evaluate};
use crate::token::TokenValue;

pub struct NumberLiteral {
    ast_leaf: AstLeaf,
}

impl NumberLiteral {
    ast_leaf_default_new! {NumberLiteral,NUMBER }
}

ast_leaf_default_impl! {NumberLiteral,TokenNumber}

ast_leaf_factory_default_impl! {NumberLiteralFactory,NumberLiteral}

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