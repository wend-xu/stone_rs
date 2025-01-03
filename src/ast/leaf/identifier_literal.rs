use crate::ast::ast_leaf::AstLeaf;
use crate::{ast_leaf_factory_default_impl, ast_leaf_default_impl, ast_leaf_default_new};
use crate::eval::environment::EnvWrapper;
use crate::eval::eval::{EvalRes, Evaluate};
use crate::token::TokenValue;

pub struct IdentifierLiteral {
    ast_leaf: AstLeaf,
}

impl IdentifierLiteral {
    ast_leaf_default_new! {IdentifierLiteral,IDENTIFIER }
}

ast_leaf_default_impl! {IdentifierLiteral,TokenIdentifier}

ast_leaf_factory_default_impl! {IdentifierLiteralFactory,IdentifierLiteral}

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