#[macro_export]
macro_rules! default_ast_leaf {
    ($ident:ident) => {
       impl Deref for $ident {
            type Target = AstLeaf;

            fn deref(&self) -> &Self::Target {
                &self.ast_leaf
            }
        }

       impl AstTree for $ident {
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