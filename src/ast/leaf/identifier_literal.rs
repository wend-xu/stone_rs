use crate::ast::ast_leaf::AstLeaf;
use crate::ast::ast_tree::AstTree;
use crate::eval::environment::{Env, EnvWrapper};
use crate::eval::eval::{EvalRes, Evaluate};
use crate::token::TokenValue;
use crate::{ast_leaf_default_impl, ast_leaf_default_new, ast_leaf_factory_default_impl};
use crate::parser::element::IdToken;
use crate::token::token_identifier::TokenIdentifier;

#[derive(Debug, Clone, PartialEq)]
pub struct IdentifierLiteral {
    ast_leaf: AstLeaf,
}

impl IdentifierLiteral {

    ast_leaf_default_new! {IdentifierLiteral,IDENTIFIER }

    pub fn new_with_str(str: &str) -> Box<Self> {
        IdentifierLiteral::new(TokenIdentifier::new(0,str))
    }

    pub fn id_name(&self) -> String {
        match self.leaf_val() {
            TokenValue::IDENTIFIER(id_name) => {
                id_name.clone()
            },
            _ => { panic!("[IdentifierLiteral]identifier literal value is not a TokenValue::IDENTIFIER") },
        }
    }

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