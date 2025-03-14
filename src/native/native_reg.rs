use crate::eval::environment::EnvWrapper;
use crate::native::log::Log;
use crate::native::native_fun::reg_native_fun;

pub fn native_reg(env: &mut EnvWrapper) -> Result<(), &'static str> {
    reg_native_fun(Log::new().to_box(), env)
}