use crate::ast::ast_leaf::AstLeaf;
use crate::eval::eval::{EvalRes, Evaluate};
use crate::{ast_leaf_default_eval_impl, ast_leaf_default_impl, ast_leaf_default_new, ast_leaf_factory_default_impl};
use crate::ast::ast_tree::AstTree;
use crate::eval::environment::{Env, EnvWrapper};
use crate::token::TokenValue;

pub struct IdentifierLiteral {
    ast_leaf: AstLeaf,
}

impl IdentifierLiteral {
    pub fn id_name(&self) -> String {
        match self.leaf_val() {
            TokenValue::IDENTIFIER(id_name) => {
                id_name.clone()
            },
            _ => { panic!("[IdentifierLiteral]identifier literal value is not a TokenValue::IDENTIFIER") },
        }
    }

    ast_leaf_default_new! {IdentifierLiteral,IDENTIFIER }
}

ast_leaf_default_impl! {IdentifierLiteral,TokenIdentifier}

ast_leaf_factory_default_impl! {IdentifierLiteralFactory,IdentifierLiteral}

/// 从env 中获取实际的值，复制返回
impl Evaluate for IdentifierLiteral {
    fn do_eval(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        let token = self.leaf_val();
        let eval_res = match token {
            TokenValue::IDENTIFIER(id) => {
                env.get_ref(id)?
            }
            _ => {
                panic!("[{}] hold token must a TokenValue::{} , not match \
                        \n error may occur in build AstTree", stringify!( IdentifierLiteral ), stringify!( IDENTIFIER ))
            }
        };
        Ok(eval_res.clone())
    }
}