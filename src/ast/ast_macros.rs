#[macro_export]
macro_rules! ast_leaf_new_for {
    ($node_name:ident, $token_value_type:ident) => {
      ast_leaf_new_for!($node_name, $token_value_type, ast_leaf );
    };

    ($node_name:ident, $token_value_type:ident, $field_name:ident) => {
        pub fn new(token: Box<dyn Token>) -> Box<$node_name> {
             match token.value() {
                 TokenValue::$token_value_type(_) => {
                    Box::new($node_name { ast_leaf: AstLeaf::new_un_ref(token) })
                }
                _ => {
                    panic!("[{}] token value should should a [{}],actual is [{:?}]",
                        stringify!($node_name),stringify!($token_value_type),token.value())
                }
            }
        }

        pub fn is_match(token: &Box<dyn Token>) -> bool {
            match token.value() {
                TokenValue::$token_value_type(_) => { true }
                _ => { false }
            }
        }
    }
}

#[macro_export]
macro_rules! ast_leaf_impl_for {
    ($node_name:ident, $token_name:ident) => {
        ast_leaf_impl_for!($node_name, $token_name, ast_leaf );
    };

    ($node_name:ident, $token_name:ident, $field_name:ident) => {
       impl AstTree for $node_name {
            fn child(&self, index: usize) -> Option<&Box<dyn AstTree>> {
                self.$field_name.child(index)
            }

            fn num_children(&self) -> usize {
                self.$field_name.num_children()
            }

            fn children(&self) -> Iter<Box<dyn AstTree>> {
                self.$field_name.children()
            }

            fn location(&self) -> String {
                self.$field_name.location()
            }

            fn actual_type_id(&self) -> TypeId {
                TypeId::of::<$node_name>()
            }
       }
    };
}

#[macro_export]
macro_rules! ast_list_new_for {
    ($node_name:ident) => {
      ast_list_new_for!($node_name, children );
    };

    ($node_name:ident, $field_name:ident) => {
        pub fn new($field_name: Vec<Box<dyn AstTree>>) -> $node_name {
            $node_name{
                 $field_name:AstList {
                     node_name:stringify!($node_name),
                     children:$field_name
                 }
            }
        }
    }
}

#[macro_export]
macro_rules! ast_list_impl_for {
    ($node_name:ident) => {
        ast_list_impl_for!($node_name, children );
    };

    ($node_name:ident, $field_name:ident) => {
       impl AstTree for $node_name {
            fn child(&self, index: usize) -> Option<&Box<dyn AstTree>> {
                self.$field_name.child(index)
            }

            fn num_children(&self) -> usize {
                self.$field_name.num_children()
            }

            fn children(&self) -> Iter<Box<dyn AstTree>> {
                self.$field_name.children()
            }

            fn location(&self) -> String {
                self.$field_name.location()
            }

            fn actual_type_id(&self) -> TypeId {
                TypeId::of::<$node_name>()
            }
       }
    };
}


#[macro_export]
macro_rules! generate {
    ($($name:ident),+) => {
        $(
           pub struct $name {
                children:AstList
            }

            impl $name {
                ast_list_new_for!{$name}
            }

            ast_list_impl_for!{$name}
        )+
    };
}



/// 构建终结符 的 Element 通用实现
#[macro_export]
macro_rules! ast_impl_element_terminal {
    ($ele_name:ident,$node_name:ident,$def_factory_name:ident) => {
       pub struct $ele_name{
          factory: Box<dyn AstLeafFactory>,
       }

       impl $ele_name {
           pub fn new(factory:Option<Box<dyn AstLeafFactory>>) -> Box<Self>{
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

       impl Element for $ele_name {
            fn parse(&self, lexer: &mut dyn Lexer, res: &mut Vec<Box<dyn AstTree>>) -> Result<(), String> {
                let read = lexer.read().unwrap();
                res.push($node_name::new(read));
                Ok(())
            }

            fn is_match(&self,lexer: &mut dyn Lexer) -> bool {
                 if let Some(box_token) = lexer.peek(0){
                    $node_name::is_match(box_token)
                 }else { false }
            }
        }
    }
}


#[macro_export]
macro_rules! ast_impl_leaf_factory {
   ($factory_name:ident,$node_name:ident) => {
      pub struct $factory_name {}

      impl $factory_name {
        pub fn new() -> Box<Self> {
            Box::new($factory_name {})
        }
      }

      impl AstLeafFactory for $factory_name {
          fn make(&self, res: Box<dyn Token>) -> Box<dyn AstTree> {
              $node_name::new(res)
          }
      }
   }
}


#[macro_export]
macro_rules! ast_impl_list_factory {
   ($factory_name:ident,$node_name:ident) => {
      pub struct $factory_name {}

      impl $factory_name {
        pub fn new() -> Box<Self> {
            Box::new($factory_name {})
        }
      }

      impl AstFactory for $factory_name {
          fn make(&self, res: Vec<Box<dyn AstTree>>) -> Box<dyn AstTree> {
              Box::new($node_name::new(res))
          }
      }
   }
}