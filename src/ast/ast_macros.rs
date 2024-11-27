#[macro_export]
macro_rules! ast_leaf_new_for {
    ($node_name:ident, $token_name:ident) => {
      ast_leaf_new_for!($node_name, $token_name, ast_leaf );
    };

    ($node_name:ident, $token_name:ident, $field_name:ident) => {
        pub fn new(token: Box<$token_name>) -> Box<$node_name> {
            Box::new($node_name { $field_name: AstLeaf::new(token) })
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