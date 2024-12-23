use crate::ast::ast_leaf::{AstLeaf, IdentifierLiteral, NumberLiteral, StringLiteral};
use crate::ast::ast_list::{AstList, BinaryExpr, BlockStmt, IfStmt, NegativeExpr, NullStmt, PrimaryExpr, WhileStmt};
use crate::ast::ast_tree::AstTree;
use crate::{ast_impl_leaf_factory, ast_impl_list_factory};
use crate::token::Token;

pub trait AstFactory {
    fn make(&self, res:Vec<Box<dyn AstTree>>) -> Box<dyn AstTree>;
}


pub trait AstLeafFactory {
    fn make(&self, res: Box<dyn Token>) -> Box<dyn AstTree>;
}
pub struct AstListFactory {}
impl AstListFactory {
    pub fn new() -> Box<Self> {
        Box::new(AstListFactory {})
    }
}
impl AstFactory for AstListFactory {
    fn make(&self, res: Vec<Box<dyn AstTree>>) -> Box<dyn AstTree> {
        let mut res = res;
        if res.len() == 1 {
           res.remove(0)
        }else { Box::new(AstList::new(res)) }
    }
}

// 宏 构建 ast_list的工厂类
ast_impl_list_factory! {BinaryExprFactory,BinaryExpr}
ast_impl_list_factory! {BlockStmtFactory,BlockStmt}
ast_impl_list_factory! {IfStmtFactory,IfStmt}
ast_impl_list_factory! {NegativeExprFactory,NegativeExpr}
ast_impl_list_factory! {NullStmtFactory,NullStmt}
ast_impl_list_factory! {PrimaryExprFactory,PrimaryExpr}
ast_impl_list_factory! {WhileStmtFactory,WhileStmt}

// 宏 构建 ast_leaf的工厂类
ast_impl_leaf_factory! {DefAstLeafFactory,AstLeaf}
ast_impl_leaf_factory! {NumberLiteralFactory,NumberLiteral}
ast_impl_leaf_factory! {IdentifierLiteralFactory,IdentifierLiteral}
ast_impl_leaf_factory! {StringLiteralFactory,StringLiteral}
