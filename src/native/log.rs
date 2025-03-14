use crate::ast::list::paramter_list::ParameterList;
use crate::eval::environment::{Env, EnvWrapper};
use crate::eval::eval::EvalRes;
use crate::native::native_fun::NativeFun;
use crate::param_list;

pub struct Log;

impl Log {
    pub fn new() -> Self {
        Log {}
    }

    fn log(msg: &str) -> EvalRes {
        println!("log => {}", msg);
        EvalRes::VOID
    }

    pub fn to_box(self) -> Box<Log> {
        Box::new(self)
    }
}

impl NativeFun for Log {
    fn reg_name(&self) -> &'static str {
        "log"
    }

    fn param_list(&self) -> ParameterList {
        param_list!["msg"]
    }

    fn do_eval(&self, env_wrapper: &mut EnvWrapper) -> Result<EvalRes, String> {
        let msg = env_wrapper.get_ref("msg")?.to_string();
        Ok(Log::log(&msg))
    }

    fn clone_native_fun(&self) -> Box<dyn NativeFun> {
        Log::new().to_box()
    }
}