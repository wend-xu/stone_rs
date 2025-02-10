use crate::ast::ast_list::AstList;
use crate::{ast_list_default_impl, ast_list_default_new, ast_list_factory_default_impl};
use crate::eval::environment::EnvWrapper;
use crate::eval::eval::{EvalRes, Evaluate};

struct ParameterList {
    children: AstList
}

impl ParameterList {
    ast_list_default_new!{ ParameterList }
}

// ast_list_default_impl!{ ParameterList }
impl crate::ast::ast_tree::AstTree for ParameterList {
    fn child(&self, index: usize) -> Option<&Box<dyn crate::ast::ast_tree::AstTree>> {
        self.children.child(index)
    }

    fn num_children(&self) -> usize {
        self.children.num_children()
    }

    fn children(&self) -> std::slice::Iter<Box<dyn crate::ast::ast_tree::AstTree>> {
        self.children.children()
    }

    fn location(&self) -> String {
        self.children.location()
    }

    fn actual_type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<ParameterList>()
    }

    fn eval(&self) -> Box<&dyn crate::eval::eval::Evaluate> {
        Box::new(self)
    }
}
ast_list_factory_default_impl!{ ParameterListFactory,ParameterList }

impl Evaluate for ParameterList {
    fn do_eval(&self, env: &mut EnvWrapper) -> Result<EvalRes, String> {
        todo!()
    }
}