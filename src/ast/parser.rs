use crate::ast::element::Element;

struct Parser {
    elements:Vec<Box<dyn Element>>,
}