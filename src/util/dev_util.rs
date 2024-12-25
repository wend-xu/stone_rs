use std::any::TypeId;
use std::fmt::Debug;
use std::io;
use std::io::Write;
use crate::ast::element::{*};

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

pub fn element_name(type_id:TypeId) -> &'static str {
    if type_id == TypeId::of::<Tree>() {
        return "Tree";
    } else if type_id == TypeId::of::<OrTree>(){
        return "OrTree";
    }else if type_id == TypeId::of::<Repeat>(){
        return "Repeat";
    }else if type_id == TypeId::of::<Leaf>(){
        return "Leaf";
    }else if type_id == TypeId::of::<IdToken>(){
        return "IdToken";
    }else if type_id == TypeId::of::<StrToken>(){
        return "StrToken";
    }else if type_id == TypeId::of::<NumToken>(){
        return "NumToken";
    }else if type_id == TypeId::of::<Skip>(){
        return "Skip";
    }else if type_id == TypeId::of::<Expr>(){
        return "Expr";
    }else{
        return "Error";
    }
}