use crate::ast::ast_tree::AstTree;
use crate::eval::eval::Evaluate;
use crate::util::str_util::{lines_concat_with_divide, wrapper_node_name, wrapper_sub_block};
use std::any::{Any, TypeId};
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::slice::Iter;
use crate::ast::list::paramter_list::ParameterList;

/// 宏展开生成代码：
/// generate![BinaryExpr,BlockStmt,IfStmt,NegativeExpr,NullStmt,PrimaryExpr,WhileStmt];
pub struct AstList {
    node_name: &'static str,
    children: Vec<Box<dyn AstTree>>,
}

impl AstList {
    pub fn new_def(children: Vec<Box<dyn AstTree>>) -> AstList {
        AstList {
            node_name: "ast_list",
            children,
        }
    }

    pub fn new(node_name: &'static str, children: Vec<Box<dyn AstTree>>) -> AstList {
        AstList {
            node_name,
            children,
        }
    }

    pub fn child_as_eval(&self, index: usize, err_msg: String) -> Result<Box<&dyn Evaluate>, String> {
        match self.children.get(index) {
            None => {
                Err(format!("Child is None, could not cast to Evaluate, index: {} ,\
            caller err msg :{}", index, err_msg))
            }
            Some(tree_node) => {
                tree_node.actual_type_id();
                Ok(tree_node.eval())
            }
        }
    }

    pub fn child_downcast<T: AstTree + Clone + 'static>(&self, index: usize) -> Result<T, String> {
        match self.child(index) {
            None => {
                Err(format!("is none in index {index}"))
            }
            Some(tree_node) => {
                println!("child_downcast\n{}", tree_node.location());
                if tree_node.actual_type_id() == TypeId::of::<T>() {

                    Ok((*tree_node.to_any().downcast_ref::<T>().unwrap()).clone())
                } else { Err("not the target type".to_string()) }
            }
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


    fn eval(&self) -> Box<&dyn Evaluate> {
        panic!("[AstList][eval] unsupported eval type");
    }

    fn to_any(&self) -> &dyn Any {
        self
    }

    fn clone_tree(&self) -> Box<dyn AstTree> {
        Box::new(self.clone())
    }

    fn eq_tree(&self, other: &dyn AstTree) -> bool {
        if other.actual_type_id() == TypeId::of::<AstList>() {
            self == other.to_any().downcast_ref::<AstList>().unwrap()
        } else { false }
    }
}

impl Debug for AstList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.location())
    }
}

impl Clone for AstList {
    fn clone(&self) -> Self {
        let mut child_clone: Vec<Box<dyn AstTree>> = vec![];

        for child_one in &self.children {
            child_clone.push(child_one.clone_tree());
        }

        AstList::new(self.node_name, child_clone)
    }
}

impl PartialEq for AstList {
    fn eq(&self, other: &Self) -> bool {
        let self_children = &self.children;
        let other_children = &other.children;
        if self_children.len() != other_children.len()
            || self.node_name != other.node_name {
            false
        } else {
            for i in 0..self_children.len() {
                if !self_children[i].eq_tree(other_children[i].deref()) { return false; }
            }
            true
        }
    }
}

















