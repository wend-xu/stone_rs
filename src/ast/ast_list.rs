use crate::ast::ast_tree::AstTree;
use crate::util::str_util::{lines_concat_with_divide, wrapper_node_name, wrapper_sub_block};
use crate::{ast_list_impl_for, ast_list_new_for};
use std::any::TypeId;
use std::fmt::Debug;
use std::slice::Iter;
use crate::ast::eval::Evaluate;

pub struct AstList {
    node_name: &'static str,
    children: Vec<Box<dyn AstTree>>,
}

impl AstList {
    pub fn new(children: Vec<Box<dyn AstTree>>) -> AstList {
        AstList {
            node_name:"ast_list",
            children
        }
    }
}

impl AstTree for AstList {
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
        let node_name = wrapper_node_name(self.node_name.to_string());
        let mut child = self.children();

        let mut sub_block_vec: Vec<String> = vec![];
        while let Some(child) = child.next() {
            sub_block_vec.push(child.location());
        }

        let sub_block = lines_concat_with_divide(sub_block_vec, Some("    "));

        wrapper_sub_block(node_name, sub_block)
    }

    fn actual_type_id(&self) -> TypeId {
        TypeId::of::<AstList>()
    }
}

/// 宏展开生成代码：
/// generate![BinaryExpr,BlockStmt,IfStmt,NegativeExpr,NullStmt,PrimaryExpr,WhileStmt];

pub struct BinaryExpr {
    children: AstList,
}

impl BinaryExpr {
    ast_list_new_for! {BinaryExpr}
}

impl AstTree for BinaryExpr {
    fn child(&self, index: usize) -> Option<&Box<dyn AstTree>> {
        self.children.child(index)
    }

    fn num_children(&self) -> usize {
        self.children.num_children()
    }

    fn children(&self) -> Iter<Box<dyn AstTree>> {
        self.children.children()
    }

    fn location(&self) -> String {
        self.children.location()
    }

    fn actual_type_id(&self) -> TypeId {
        TypeId::of::<BinaryExpr>()
    }

    fn eval(&self) -> Box<&dyn Evaluate> {
        Box::new(self)
    }
}

pub struct BlockStmt {
    children: AstList,
}

impl BlockStmt {
    ast_list_new_for! {BlockStmt}
}

ast_list_impl_for! {BlockStmt}

pub struct IfStmt {
    children: AstList,
}

impl IfStmt {
    ast_list_new_for! {IfStmt}
}

ast_list_impl_for! {IfStmt}


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

