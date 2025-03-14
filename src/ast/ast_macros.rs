
#[macro_export]
macro_rules! ast_leaf_default_new {
    ($node_name:ident, $token_value_type:ident) => {
      ast_leaf_default_new!($node_name, $token_value_type, ast_leaf );
    };

    ($node_name:ident, $token_value_type:ident, $field_name:ident) => {
        pub fn new(token: Box<dyn crate::token::Token>) -> Box<$node_name> {
             match token.value() {
                 crate::token::TokenValue::$token_value_type(_) => {
                    Box::new($node_name { ast_leaf: crate::ast::ast_leaf::AstLeaf::new_un_ref(token) })
                }
                _ => {
                    panic!("[{}] token value should should a [{}],actual is [{:?}]",
                        stringify!($node_name),stringify!($token_value_type),token.value())
                }
            }
        }

        pub fn is_match(token: &Box<dyn crate::token::Token>) -> bool {
            match token.value() {
                crate::token::TokenValue::$token_value_type(_) => { true }
                _ => { false }
            }
        }

        pub fn leaf_val(&self) -> &crate::token::TokenValue{
            self.ast_leaf.token().value()
        }
    }
}

#[macro_export]
macro_rules! ast_leaf_default_impl {
    ($node_name:ident, $token_name:ident) => {
        ast_leaf_default_impl!($node_name, $token_name, ast_leaf );
    };

    ($node_name:ident, $token_name:ident, $field_name:ident) => {
       impl crate::ast::ast_tree::AstTree for $node_name {
            fn child(&self, index: usize) -> Option<&Box<dyn crate::ast::ast_tree::AstTree>> {
                self.$field_name.child(index)
            }

            fn num_children(&self) -> usize {
                self.$field_name.num_children()
            }

            fn children(&self) -> std::slice::Iter<Box<dyn crate::ast::ast_tree::AstTree>> {
                self.$field_name.children()
            }

            fn location(&self) -> String {
                self.$field_name.location()
            }

            fn actual_type_id(&self) -> std::any::TypeId {
                std::any::TypeId::of::<$node_name>()
            }

            fn eval(&self) -> Box<&dyn crate::eval::eval::Evaluate> {
                Box::new(self)
            }

            fn to_any(& self) -> &dyn std::any::Any{
                self
            }

            fn clone_tree(&self) -> Box<dyn crate::ast::ast_tree::AstTree> {
                std::boxed::Box::new(self.clone())
            }

            fn eq_tree(&self, other: &dyn crate::ast::ast_tree::AstTree) -> bool {
                if self.actual_type_id() == std::any::TypeId::of::<$node_name>() {
                    self.ast_leaf == other.to_any().downcast_ref::<$node_name>().unwrap().ast_leaf
                }else { false }
            }
       }
    };
}

#[macro_export]
macro_rules! ast_list_default_new {
    ($node_name:ident) => {
      ast_list_default_new!($node_name, children );
    };

    ($node_name:ident, $field_name:ident) => {
        pub fn new($field_name: Vec<Box<dyn  crate::ast::ast_tree::AstTree>>) -> $node_name {
            $node_name{
                 $field_name:crate::ast::ast_list::AstList::new(
                     stringify!($node_name),$field_name
                 )
            }
        }
    }
}

#[macro_export]
macro_rules! ast_list_default_impl {
    ($node_name:ident) => {
        ast_list_default_impl!($node_name, children );
    };

    ($node_name:ident, $field_name:ident) => {
       impl  crate::ast::ast_tree::AstTree for $node_name {
            fn child(&self, index: usize) -> Option<&Box<dyn  crate::ast::ast_tree::AstTree>> {
                self.$field_name.child(index)
            }

            fn num_children(&self) -> usize {
                self.$field_name.num_children()
            }

            fn children(&self) -> std::slice::Iter<Box<dyn  crate::ast::ast_tree::AstTree>> {
                self.$field_name.children()
            }

            fn location(&self) -> String {
                self.$field_name.location()
            }

            fn actual_type_id(&self) -> std::any::TypeId {
                std::any::TypeId::of::<$node_name>()
            }

            fn eval(&self) -> Box<&dyn crate::eval::eval::Evaluate> {
                Box::new(self)
            }

            fn to_any(& self) -> &dyn std::any::Any{
                self
            }

            fn clone_tree(&self) -> Box<dyn crate::ast::ast_tree::AstTree> {
                let self_clone = (*self).clone();
                Box::new(self_clone)
            }

            fn eq_tree(&self, other:&dyn crate::ast::ast_tree::AstTree) -> bool {
                 if other.actual_type_id() == std::any::TypeId::of::<$node_name>() {
                     self == other.to_any().downcast_ref::<$node_name>().unwrap()
                 }else{ false }
            }
       }
    };
}


#[macro_export]
macro_rules! generate {
    ($($name:ident),+) => {
        $(
           pub struct $name {
                children:crate::ast::ast_list::AstList
            }

            impl $name {
                ast_list_default_new!{$name}
            }

            ast_list_default_impl!{$name}
        )+
    };
}



/// 构建终结符 的 Element 通用实现
#[macro_export]
macro_rules! ast_element_terminal_default_impl {
    ($ele_name:ident,$node_name:ident,$def_factory_name:ident) => {
       pub struct $ele_name{
          factory: Box<dyn crate::parser::factory::AstLeafFactory>,
       }

       impl $ele_name {
           pub fn new(factory:Option<Box<dyn crate::parser::factory::AstLeafFactory>>) -> Box<Self>{
                let factory = match factory{
                    None => { $def_factory_name::new() }
                    Some(factory) => { factory }
                };
                Box::new($ele_name{factory})
            }

            pub fn new_def() -> Box<Self>{
                Self::new(None)
            }
       }

       impl crate::parser::element::Element for $ele_name {
            fn parse(&self, lexer: &mut dyn Lexer, res: &mut Vec<Box<dyn  crate::ast::ast_tree::AstTree>>) -> Result<(), String> {
                let read = lexer.read().unwrap();
                res.push($node_name::new(read));
                Ok(())
            }

            fn is_match(&self,lexer: &mut dyn Lexer) -> bool {
                 if let Some(box_token) = lexer.peek(0){
                    $node_name::is_match(box_token)
                 }else { false }
            }

            ast_element_actual_type_id!{}
        }
    }
}

/// Element 获取实际类型ID
#[macro_export]
macro_rules! ast_element_actual_type_id{
    () => {
        fn el_actual_type_id(&self) ->  std::any::TypeId  {
             std::any::TypeId::of::<Self>()
        }
    }
}


#[macro_export]
macro_rules! ast_leaf_factory_default_impl {
   ($factory_name:ident,$node_name:ident) => {
      pub struct $factory_name {}

      impl $factory_name {
        pub fn new() -> Box<Self> {
            Box::new($factory_name {})
        }
      }

      impl crate::parser::factory::AstLeafFactory for $factory_name {
          fn make(&self, res: Box<dyn crate::token::Token>) -> Box<dyn  crate::ast::ast_tree::AstTree> {
              $node_name::new(res)
          }
      }
   }
}


#[macro_export]
macro_rules! ast_list_factory_default_impl {
   ($factory_name:ident,$node_name:ident) => {
      #[derive(Clone,Copy)]
      pub struct $factory_name {}

      impl $factory_name {
        pub fn new() -> Box<Self> {
            Box::new($factory_name {})
        }
      }

      impl crate::parser::factory::AstFactory for $factory_name {
          fn make(&self, res: Vec<Box<dyn  crate::ast::ast_tree::AstTree>>) -> Box<dyn  crate::ast::ast_tree::AstTree> {
              Box::new($node_name::new(res))
          }

          fn clone(&self) -> Box<dyn crate::parser::factory::AstFactory> {
            Box::new(Clone::clone(self))
          }
      }
   }
}


#[macro_export]
macro_rules! number_compute {
   ( $left:ident, $right:ident, $op:ident; [$($op_calc:tt),+]; [$($op_eq:tt),+]) => {
        match $op {
        $(
            stringify!($op_calc) => {
                crate::eval::eval::EvalRes::NUMBER($left $op_calc $right)
            }
        )+
        $(
            stringify!($op_eq) => {
                crate::eval::eval::EvalRes::BOOLEAN($left $op_eq $right)
            }
        )+
            &_ => {
                panic!("[BinaryExpr] {} is not legal operator",$op)
            }
        }
   }
}

#[macro_export]
macro_rules! ast_leaf_default_eval_impl {
    ($node_name:ident,$token_val:ident ,$eval_res:ident) => {
        impl crate::eval::eval::Evaluate for $node_name{
            fn do_eval(&self, env: &mut crate::eval::environment::EnvWrapper) -> Result<crate::eval::eval::EvalRes, String> {
                let token = self.leaf_val();
                let eval_res = match token {
                    crate::token::TokenValue::$token_val(id) => {
                        EvalRes::$eval_res(id.clone())
                    }
                    _ => {
                        panic!("[{}] hold token must a TokenValue::{} , not match \
                        \n error may occur in build AstTree",stringify!($node_name),stringify!($token_val))
                    }
                };
                Ok(eval_res)
            }
        }
    };
}

#[macro_export]
macro_rules! param_list {
    ($($param_name:tt)+) => {
        {
            let mut params = vec![];
            $(
              params.push($param_name);
            )+
            ParameterList::new_with_name(params)
        }
    }
}