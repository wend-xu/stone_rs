use crate::ast::ast_tree::AstTree;
use crate::util::str_util::{lines_concat_with_divide, wrapper_node_name, wrapper_sub_block};
use std::any::TypeId;
use std::fmt::Debug;
use std::slice::Iter;


/// 宏展开生成代码：
/// generate![BinaryExpr,BlockStmt,IfStmt,NegativeExpr,NullStmt,PrimaryExpr,WhileStmt];
pub struct AstList {
    node_name: &'static str,
    children: Vec<Box<dyn AstTree>>,
}

impl AstList {
    pub fn new_def(children: Vec<Box<dyn AstTree>>) -> AstList {
        AstList {
            node_name:"ast_list",
            children
        }
    }

    pub fn new(node_name:&'static str ,children: Vec<Box<dyn AstTree>>) -> AstList {
        AstList {
            node_name,
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
















