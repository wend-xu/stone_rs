use crate::ast::ast_leaf::AstLeaf;
use crate::{ast_impl_leaf_factory, ast_leaf_impl_for, ast_leaf_new_for};
use crate::eval::environment::EnvWrapper;
use crate::eval::eval::{EvalRes, Evaluate};
use crate::token::TokenValue;

pub struct StringLiteral {
    ast_leaf: AstLeaf,
}

impl StringLiteral {
    ast_leaf_new_for! {StringLiteral,StringVal }
}

ast_leaf_impl_for! {StringLiteral,TokenText}

ast_impl_leaf_factory! {StringLiteralFactory,StringLiteral}

impl Evaluate for StringLiteral {
    fn do_eval(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        let token = self.leaf_val();
        let eval_res = match token {
            TokenValue::StringVal(id) => {
                EvalRes::StringVal(id.clone())
            }
            _ => {
                panic!("[StringLiteral] hold token must a TokenValue::StringVal , not match \n error may occur in build AstTree")
            }
        };
        Ok(eval_res)
    }
}
