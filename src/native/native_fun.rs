use std::fmt::{Debug, Formatter};
use crate::ast::list::paramter_list::ParameterList;
use crate::eval::environment::{Env, EnvWrapper};
use crate::eval::eval::EvalRes;

pub trait NativeFun {
    fn reg_name(&self) -> &'static str;

    fn param_list(&self) -> ParameterList;

    fn do_eval(&self, env_wrapper: &mut EnvWrapper) -> Result<EvalRes, String>;

    fn clone_native_fun(&self) -> Box<dyn NativeFun>;
}

impl Debug for Box<dyn NativeFun> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<NativeFun:{}>", self.reg_name())
    }
}

impl Clone for Box<dyn NativeFun> {
    fn clone(&self) -> Self {
        self.clone_native_fun()
    }
}


impl PartialEq for Box<dyn NativeFun> {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}


pub fn reg_native_fun(native_fun: Box<dyn NativeFun>, env: &mut EnvWrapper) -> Result<(), &'static str> {
    let reg_name = native_fun.reg_name();
    let native_fun_res = EvalRes::NativeFun(reg_name.to_string(), native_fun);
    env.put(reg_name.to_string(), native_fun_res)
}
