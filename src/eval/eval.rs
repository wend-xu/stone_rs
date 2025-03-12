use crate::ast::ast_tree::AstTree;
use crate::ast::list::block_stmt::BlockStmt;
use crate::ast::list::paramter_list::ParameterList;
use crate::eval::environment::EnvWrapper;

#[derive(Debug, Clone, PartialEq)]
pub enum EvalRes {
    NUMBER(isize),
    StringVal(String),
    NAME(String),
    BOOLEAN(bool),
    Struct(Vec<EvalRes>),
    VOID,
    FUNCTION(String, ParameterList, BlockStmt),
}

impl EvalRes {
    pub fn is_string(&self) -> bool {
        match self {
            EvalRes::StringVal(_) => { true }
            _ => { false }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            EvalRes::StringVal(string) => { string.clone() }
            EvalRes::NAME(id) => { id.clone() }
            EvalRes::BOOLEAN(boolean) => { boolean.to_string() }
            EvalRes::NUMBER(num) => { num.to_string() }
            _ => { panic!("{:?} could‘t be str", self); }
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            EvalRes::NUMBER(_) => { true }
            _ => { false }
        }
    }

    pub fn to_number(&self) -> &isize {
        match self {
            EvalRes::NUMBER(num) => { num }
            _ => { panic!("{:?} could‘t be number", self); }
        }
    }

    pub fn is_identifier(&self) -> bool {
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
            EvalRes::FUNCTION(_, _, _) => { false }
        }
    }
}


pub trait Evaluate {
    fn do_eval(&self, env: &mut EnvWrapper) -> Result<EvalRes, String>;

    fn do_eval_postfix(&self, env: &mut EnvWrapper,  result: EvalRes) -> Result<EvalRes, String> {
      panic!("[Evaluate][do_eval_postfix] just impl in postfix type")
    }
}





