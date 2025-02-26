use std::any::TypeId;
use crate::ast::ast_list::AstList;
use crate::ast::ast_tree::AstTree;
use crate::eval::environment::{Env, EnvWrapper};
use crate::eval::eval::{EvalRes, Evaluate};
use crate::{ast_list_default_impl, ast_list_default_new, ast_list_factory_default_impl, number_compute};
use crate::ast::leaf::identifier_literal::IdentifierLiteral;

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    children: AstList,
}

impl BinaryExpr {
    ast_list_default_new! {BinaryExpr}

    fn get_binary_part_and_eval(&self, env: &mut EnvWrapper, index: usize, err_part: &str) -> Result<EvalRes, String> {
        let eval = self.children.
            child_as_eval(index, format!("[BinaryExpr] {} is None", err_part))?;
        eval.do_eval(env)
    }

    fn get_id_literal(&self, index: usize, err_part: &str) -> Result<String, String> {
        let id_literal_op = self.children.child(index);
        if id_literal_op.is_none() {
            return Err(format!("[BinaryExpr] {err_part} is none,error"));
        }
        let id_literal = id_literal_op.unwrap();
        if id_literal.actual_type_id() == TypeId::of::<IdentifierLiteral>() {
            let left_actual_cast =
                id_literal.to_any().downcast_ref::<IdentifierLiteral>().unwrap();
            Ok(left_actual_cast.id_name())
        } else {
            Err(format!("[BinaryExpr] get {err_part} literal fail,{err_part} not a IdentifierLiteral"))
        }
    }

    fn left_literal(&self) -> Result<String, String> {
       self.get_id_literal(0, "left")
    }

    fn left(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        self.get_binary_part_and_eval(env, 0, "left")
    }

    fn operator(&self) -> Result<String, String> {
        self.get_id_literal( 1, "operator")
    }

    fn right(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        self.get_binary_part_and_eval(env, 2, "right")
    }

    fn compute_number(&self, left: &isize, operator: String, right: &isize) -> Result<EvalRes, String> {
        //,==,>,< 为比较运算符，运算结果是 布尔值
        let operator = operator.as_str();
        Ok(number_compute! {left,right,operator;[+,-,*,/,%];[==,>,<]})
    }

    fn compute_assign(&self, env: &mut EnvWrapper, right: EvalRes) -> Result<EvalRes, String> {
        let left_name = self.left_literal()?;
        if right == EvalRes::VOID {
            return Err(format!("[BinaryExpr] could not assign [void] for {:?} ", left_name));
        }

        env.put(left_name, right)?;
        Ok(EvalRes::VOID)
    }

    fn compute_op(&self, left: &EvalRes, operator: &str , right: &EvalRes) -> Result<EvalRes, String> {
        if left.is_number() && right.is_number() {
            self.compute_number(left.to_number(), operator.to_string(), right.to_number())
        } else if operator == "+" {
            Ok(EvalRes::StringVal(format!("{}{}", left.to_string(), right.to_string())))
        } else if operator == "==" {
            Ok(EvalRes::BOOLEAN(left == right))
        } else {
            panic!("[BinaryExpr] bad operator {:?}", operator)
        }
    }
}

ast_list_default_impl! {BinaryExpr}


ast_list_factory_default_impl! {BinaryExprFactory,BinaryExpr}

impl Evaluate for BinaryExpr {
    fn do_eval(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        let operator = self.operator()?;
        let right_val = self.right(env)?;

        if &operator == "=" {
            self.compute_assign(env, right_val)
        } else {
            let left_val = self.left(env)?;
            self.compute_op(&left_val, &operator, &right_val)
        }
    }
}
