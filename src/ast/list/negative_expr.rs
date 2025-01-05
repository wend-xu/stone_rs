use crate::ast::ast_list::AstList;
use crate::{ast_list_factory_default_impl, ast_list_default_impl, ast_list_default_new};
use crate::ast::ast_tree::AstTree;
use crate::eval::environment::EnvWrapper;
use crate::eval::eval::{EvalRes, Evaluate};

pub struct NegativeExpr {
    children: AstList,
}
impl NegativeExpr {
    ast_list_default_new! { NegativeExpr }

    fn operand(&self) -> Result< &Box<dyn AstTree>,String>{
        match self.child(0) {
            None => { Err("[NegativeExpr] operand should not be empty".to_string()) },
            Some(tree_node) => {
                Ok(tree_node)
            }
        }
    }
}
ast_list_default_impl! { NegativeExpr }

ast_list_factory_default_impl! {NegativeExprFactory,NegativeExpr}

impl Evaluate for NegativeExpr {
    fn do_eval(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        let operand_res = self.operand()?.eval().do_eval(env)?;
        if operand_res.is_number() {
            Ok(EvalRes::NUMBER(-operand_res.to_number()))
        }else {
            Err("[NegativeExpr] bad grammar, only number can negative".to_string())
        }
    }
}