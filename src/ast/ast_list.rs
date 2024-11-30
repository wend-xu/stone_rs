use crate::ast::ast_tree::AstTree;
use crate::{ast_list_impl_for, ast_list_new_for,generate};
use std::fmt::Debug;
use std::slice::Iter;

pub struct AstList {
    children: Vec<Box<dyn AstTree>>,
}

impl AstList {
    pub fn new(children: Vec<Box<dyn AstTree>>) -> AstList {
        AstList {
            children
        }
    }
}

impl AstTree  for AstList {
    fn child(&self, index: usize) -> Option<&Box<dyn AstTree>> {
        self.children.get(index)
    }

    fn num_children(&self) -> usize {
        self.children.len()
    }

    fn children(&self) -> Iter<Box<dyn AstTree>> {
        self.children.iter()
    }

    fn location(&self) -> String {
        "todo ...".to_string()
    }
}

/// 宏展开生成代码：
/// generate![BinaryExpr,BlockStmt,IfStmt,NegativeExpr,NullStmt,PrimaryExpr,WhileStmt];

pub struct BinaryExpr{
    children:AstList
}

impl BinaryExpr {
    ast_list_new_for!{BinaryExpr}
}

ast_list_impl_for!{BinaryExpr}

pub struct BlockStmt {
    children:AstList
}

impl BlockStmt {
    ast_list_new_for!{BlockStmt}
}

ast_list_impl_for!{BlockStmt}

pub struct IfStmt {
    children:AstList
}

impl IfStmt {
    ast_list_new_for!{IfStmt}
}

ast_list_impl_for!{IfStmt}


pub struct NegativeExpr {
    children: AstList,
}
impl NegativeExpr {
    ast_list_new_for! { NegativeExpr }
}
ast_list_impl_for! { NegativeExpr }


pub struct NullStmt {
    children: AstList,
}
impl NullStmt {
    ast_list_new_for! { NullStmt }
}
ast_list_impl_for! { NullStmt }


pub struct PrimaryExpr {
    children: AstList,
}
impl PrimaryExpr {
    ast_list_new_for! { PrimaryExpr }
}
ast_list_impl_for! { PrimaryExpr }


pub struct WhileStmt {
    children: AstList,
}
impl WhileStmt {
    ast_list_new_for! { WhileStmt }
}
ast_list_impl_for! { WhileStmt }

