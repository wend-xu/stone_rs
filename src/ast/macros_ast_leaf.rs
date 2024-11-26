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
macro_rules! ast_leaf_for {
    ($node_name:ident, $token_name:ident) => {
        ast_leaf_for!($node_name, $token_name, ast_leaf );
    };

    ($node_name:ident, $token_name:ident, $field_name:ident) => {
       impl Deref for $node_name {
            type Target = AstLeaf<$token_name>;

            fn deref(&self) -> &Self::Target {
                &self.$field_name
            }
        }

       impl AstTree for $node_name {
            fn child(&self, index: usize) -> Option<Box<dyn AstTree>> {
                self.child(index)
            }

            fn num_children(&self) -> usize {
                self.num_children()
            }

            fn children(&self) -> Option<Box<dyn AstTree>> {
                self.children()
            }

            fn location(&self) -> String {
                self.location()
            }
        }
    };
}