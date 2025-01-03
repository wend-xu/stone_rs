use crate::ast::ast_leaf::AstLeaf;
use crate::{ast_impl_leaf_factory, ast_leaf_impl_for, ast_leaf_new_for};
use crate::eval::environment::EnvWrapper;
use crate::eval::eval::{EvalRes, Evaluate};
use crate::token::TokenValue;

pub struct IdentifierLiteral {
    ast_leaf: AstLeaf,
}

impl IdentifierLiteral {
    ast_leaf_new_for! {IdentifierLiteral,IDENTIFIER }
}

ast_leaf_impl_for! {IdentifierLiteral,TokenIdentifier}

ast_impl_leaf_factory! {IdentifierLiteralFactory,IdentifierLiteral}

impl Evaluate for IdentifierLiteral {
    fn do_eval(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        let token = self.leaf_val();
        let eval_res = match token {
            TokenValue::IDENTIFIER(id) => {
                EvalRes::NAME(id.clone())
            }
            _ => {
                panic!("[IdentifierLiteral] hold token must a TokenValue::IDENTIFIER , not match \n error may occur in build AstTree")
            }
        };
        Ok(eval_res)
    }
}