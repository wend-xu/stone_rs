pub trait AstTree {
    fn child(&self, index: usize) -> Option<Box<dyn AstTree>>;

    fn num_children(&self) -> usize;

    fn children(&self) -> Option<Box<dyn AstTree>>;

    fn location(&self) -> String;
}