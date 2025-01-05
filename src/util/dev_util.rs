use std::any::TypeId;
use std::fmt::Debug;
use std::io;
use std::io::Write;
use crate::parser::element::{*};

pub fn print_and_flush(msg:&str, debug:&dyn Debug) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    // 再次写入并刷新
    writeln!(handle, "{}{:?}",msg, debug).expect("Failed to write to stdout");
    handle.flush().expect("Failed to flush stdout");
}

pub fn print_and_flush_msg(msg:&str) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    // 再次写入并刷新
    writeln!(handle, "{}",msg).expect("Failed to write to stdout");
    handle.flush().expect("Failed to flush stdout");
}
