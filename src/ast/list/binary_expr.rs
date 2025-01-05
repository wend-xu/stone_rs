use crate::ast::ast_list::AstList;
use crate::ast::ast_tree::AstTree;
use crate::eval::environment::{Env, EnvWrapper};
use crate::eval::eval::{EvalRes, Evaluate};
use crate::{ast_list_default_impl, ast_list_default_new, ast_list_factory_default_impl, number_compute};

pub struct BinaryExpr {
    children: AstList,
}

impl BinaryExpr {
    ast_list_default_new! {BinaryExpr}

    fn get_binary_part(&self, env: &mut EnvWrapper, index: usize, err_part: &str) -> Result<EvalRes, String> {
        let eval = self.children.
            child_as_eval(index, format!("[BinaryExpr] {} is None", err_part))?;
        eval.do_eval(env)
    }

    fn left(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        self.get_binary_part(env, 0, "left")
    }

    fn right(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        self.get_binary_part(env, 2, "right")
    }

    fn operator(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        self.get_binary_part(env, 1, "operator")
    }

    fn compute_number(&self, left: &isize, operator: String, right: &isize) -> Result<EvalRes, String> {
        //,==,>,< 为比较运算符，运算结果是 布尔值
        let operator = operator.as_str();
        Ok(number_compute! {left,right,operator;[+,-,*,/,%];[==,>,<]})
    }

    fn compute_assign(&self, left_val: EvalRes, env: &mut EnvWrapper, right: EvalRes) -> Result<EvalRes, String> {
        let right_val = if right.is_identifier() {
            Self::compute_substitution(&right, env)?.clone()
        } else if right == EvalRes::VOID {
            return Err(format!("[BinaryExpr] could not assign [void] for {:?} ",left_val));
        } else { right };

        match left_val {
            EvalRes::NAME(name) => {
                env.put(name.clone(), right_val)?;
                Ok(EvalRes::VOID)
            }
            _ => { panic!("bad assignment,left [{:?}] not a Name", left_val) }
        }
    }

    fn compute_op(&self, left: &EvalRes, operator: &EvalRes, right: &EvalRes) -> Result<EvalRes, String> {
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

    fn compute_substitution<'a>(eval_res: &'a EvalRes, env: &'a EnvWrapper) -> Result<&'a EvalRes, String> {
        match eval_res {
            EvalRes::NAME(name) => {
                env.get_ref(name)
            }
            _ => { Ok(eval_res) }
        }
    }
}

ast_list_default_impl! {BinaryExpr}

ast_list_factory_default_impl! {BinaryExprFactory,BinaryExpr}

impl Evaluate for BinaryExpr {
    fn do_eval(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        let operator = self.operator(env)?;
        let right = self.right(env)?;
        let left = self.left(env)?;

        if &operator == "=" {
            self.compute_assign(left, env, right)
        } else {
            let left_val = Self::compute_substitution(&left, env)?;
            let right_val = Self::compute_substitution(&right, env)?;
            self.compute_op(left_val, &operator, right_val)
        }
    }
}
