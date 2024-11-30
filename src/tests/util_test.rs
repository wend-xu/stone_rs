
#[cfg(test)]
mod util_tests {
    use crate::ast::ast_list::{AstList, BinaryExpr};
    use crate::util::type_util;

    #[test]
    fn type_eq_test(){
        let ast_list = AstList::new(vec![]);
        let is_type = type_util::struct_is_type::<AstList>(&ast_list);
        println!("ast_list is AstList : {}", is_type);
        assert_eq!(is_type, true);

        let binary_expr = BinaryExpr::new(vec![]);
        let is_type = type_util::struct_is_type::<AstList>(&binary_expr);
        println!("binary_expr is AstList : {}", is_type);
        assert_eq!(is_type, false);
    }
}