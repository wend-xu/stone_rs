use std::any::TypeId;
use std::ops::Add;
use crate::ast::ast_leaf::{AstLeaf, IdentifierLiteral, NumberLiteral, StringLiteral};
use crate::ast::ast_list::BinaryExpr;
use crate::ast::ast_tree::AstTree;
use crate::ast::element::Leaf;
use crate::ast::environment::{Env, EnvWrapper};
use crate::number_compute;
use crate::token::TokenValue;

#[derive(Debug, Clone, PartialEq)]
pub enum EvalRes {
    NUMBER(isize),
    StringVal(String),
    NAME(String),
    BOOLEAN(bool),
    Struct(Vec<EvalRes>),
    VOID,
}

impl EvalRes {
    fn is_string(&self) -> bool {
        match self {
            EvalRes::StringVal(_) => { true }
            _ => { false }
        }
    }

    fn to_string(&self) -> String {
        match self {
            EvalRes::StringVal(string) => { string.clone() }
            EvalRes::NAME(id) => { id.clone() }
            EvalRes::BOOLEAN(boolean) => { boolean.to_string() }
            EvalRes::NUMBER(num) => { num.to_string() }
            _ => { panic!("{:?} could‘t be str", self); }
        }
    }

    fn is_number(&self) -> bool {
        match self {
            EvalRes::NUMBER(_) => { true }
            _ => { false }
        }
    }

    fn to_number(&self) -> &isize {
        match self {
            EvalRes::NUMBER(num) => { num }
            _ => { panic!("{:?} could‘t be number", self); }
        }
    }

    fn is_identifier(&self) -> bool {
        match self {
            EvalRes::NAME(_) => { true }
            _ => { false }
        }
    }
}

impl PartialEq<str> for EvalRes {
    fn eq(&self, other: &str) -> bool {
        match self {
            EvalRes::NAME(id) => { id.as_str() == other }
            EvalRes::StringVal(str) => { str.as_str() == other }
            EvalRes::NUMBER(_) => { false }
            EvalRes::BOOLEAN(_) => { false }
            EvalRes::Struct(_) => { false }
            EvalRes::VOID => { false }
        }
    }
}


pub trait Evaluate {
    fn do_eval(&self, env: &mut EnvWrapper) -> Result<EvalRes, String>;
}

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

impl BinaryExpr {
    fn get_binary_part(&self, env: &mut EnvWrapper, index: usize, err_part: &str) -> Result<EvalRes, String> {
        let tree_node_op = self.child(index);
        match tree_node_op {
            None => { Err(format!("[BinaryExpr] {} is None", err_part)) }
            Some(tree_node) => {
                let eval_res = tree_node.eval().do_eval(env)?;
                Ok(eval_res)
            }
        }
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

impl Evaluate for BinaryExpr {
    fn do_eval(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        let operator = self.operator(env)?;
        let right = self.right(env)?;
        let left = self.left(env)?;

        if &operator == "=" {
            // let right_val =  Self::compute_substitution(&right, env)?.clone();
            self.compute_assign(left, env, right)
        } else {
            let left_val = Self::compute_substitution(&left, env)?;
            let right_val = Self::compute_substitution(&right, env)?;
            self.compute_op(left_val, &operator, right_val)
        }
    }
}


