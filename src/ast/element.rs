use crate::ast::ast_leaf::{AstLeaf, IdentifierLiteral, NumberLiteral, StringLiteral};
use crate::ast::ast_list::AstList;
use crate::ast::ast_tree::{AstTree};
use crate::ast::parser::{Parser};
use crate::ast_impl_element_terminal;
use crate::lexer::lexer::Lexer;
use crate::token::{Token, TokenValue};
use std::any::TypeId;
use std::collections::HashMap;
use std::rc::Rc;
use crate::ast::factory::{AstFactory, AstLeafFactory, IdentifierLiteralFactory, NumberLiteralFactory, StringLiteralFactory};

pub trait Element {
    fn parse(&self, lexer: &mut dyn Lexer, res: &mut Vec<Box<dyn AstTree>>) -> Result<(), String>;

    fn is_match(&self, lexer: &mut dyn Lexer) -> bool { false }
}


pub struct Tree {
    parser: Rc<Parser>,
}

impl Tree {
    pub fn new(parser: &Rc<Parser>) -> Box<Self> {
        Box::new(Tree { parser: Rc::clone(parser) })
    }
}


impl Element for Tree {
    fn parse(&self, lexer: &mut dyn Lexer, res: &mut Vec<Box<dyn AstTree>>) -> Result<(), String> {
        let parse_res = self.parser.parse(lexer)?;
        res.push(parse_res);
        Ok(())
    }

    fn is_match(&self, lexer: &mut dyn Lexer) -> bool {
        self.parser.is_match(lexer)
    }
}

pub struct OrTree {
    parser_vec: Vec<Rc<Parser>>,
}

impl OrTree {
    pub fn new(parser_vec: Vec<Rc<Parser>>) -> Box<Self> {
        Box::new(OrTree { parser_vec })
    }

    fn choose(&self, lexer: &mut dyn Lexer) -> Option<Rc<Parser>> {
        let mut choose_tree: Option<Rc<Parser>> = None;
        while let Some(parser) = self.parser_vec.iter().next() {
            if parser.is_match(lexer) {
                choose_tree = Some(Rc::clone(parser));
                break;
            }
        }
        choose_tree
    }
}

impl Element for OrTree {
    fn parse(&self, lexer: &mut dyn Lexer, res: &mut Vec<Box<dyn AstTree>>) -> Result<(), String> {
        let choose_tree = self.choose(lexer);
        let result = if let Some(parser) = choose_tree {
            parser.parse(lexer)
        } else {
            let next_token = lexer.peek(0).unwrap();
            Err(format!("OrTree::choose failed, no parser found, token : [{} : {:?} ]", next_token.line_number(), next_token.value()))
        };

        let tree_node = result?;
        res.push(tree_node);
        Ok(())
    }

    fn is_match(&self, lexer: &mut dyn Lexer) -> bool {
        self.choose(lexer).is_some()
    }
}


pub struct Repeat {
    parser: Rc<Parser>,
    only_once: bool,
}

impl Repeat {
    pub fn new(parser: &Rc<Parser>, only_once: bool) -> Box<Self> {
        Box::new(Repeat {parser:Rc::clone(parser), only_once })
    }
}

impl Element for Repeat {
    fn parse(&self, lexer: &mut dyn Lexer, res: &mut Vec<Box<dyn AstTree>>) -> Result<(), String> {
        while self.parser.is_match(lexer) {
            /// parser 出现AstList则是factory构建ast节点的时候没有指定类型，实际上没有执行的功能
            /// 这种情况确实可以忽略，因为本身就是无法执行的，在ast树上也没意义
            ///
            /// 按照BNF的语法定义，仅在 block中有 repeat ，定义为 {(";" | EOL) [ statement ]} "}" ,
            /// 这中情况块的最后一行，会match，但是后续匹配结果为空，会触发 parse_res.num_children() = 0
            /// 看似  (";" | EOL) 作为开头很别扭，但是支持重复的模式也需要有个可匹配的模式，这样写对实现来说好像是最为简单的
            /// 若是将  (";" | EOL) 作为重复的结尾，一样可以实现匹配，相对的就是匹配完块后不进入while循环的情况
            ///
            /// 故进入while 循环后的判定条件：  不为AstList(不可执行无意义) 子节点是数为0(实际未匹配可执行内容)
            let tree_node = self.parser.parse(lexer)?;
            if tree_node.actual_type_id() == TypeId::of::<AstList>() || tree_node.num_children() > 0 {
                res.push(tree_node);
            }
            if self.only_once {
                break;
            }
        }
        Ok(())
    }

    fn is_match(&self, lexer: &mut dyn Lexer) -> bool {
        self.parser.is_match(lexer)
    }
}


/// 终结符的实现都是一样的，使用宏定义
ast_impl_element_terminal! {IdToken,IdentifierLiteral,IdentifierLiteralFactory}
ast_impl_element_terminal! {StrToken,StringLiteral,StringLiteralFactory}
ast_impl_element_terminal! {NumToken, NumberLiteral,NumberLiteralFactory}

#[derive(Debug)]
pub struct Leaf {
    tokens: Vec<TokenValue>,
}

impl Leaf {
    pub fn new(leaf_literal: Vec<&str>) -> Box<Self> {
        Box::new(
            Leaf {
                tokens: leaf_literal.iter().map(|str| TokenValue::IDENTIFIER(str.to_string())).collect()
            }
        )
    }
}

impl Element for Leaf {
    fn parse(&self, lexer: &mut dyn Lexer, res: &mut Vec<Box<dyn AstTree>>) -> Result<(), String> {
        res.push(AstLeaf::new(lexer.read().unwrap()));
        Ok(())
    }

    fn is_match(&self, lexer: &mut dyn Lexer) -> bool {
        if let Some(token_value) = lexer.peek(0) {
            self.tokens.contains(token_value.value())
        } else { false }
    }
}

pub struct Skip {
    leaf: Box<Leaf>,
}

impl Skip {
    pub fn new(leaf_literal: Vec<&str>) -> Box<Self> {
        Box::new(
            Skip {
                leaf: Leaf::new(leaf_literal)
            }
        )
    }
}

impl Element for Skip {
    fn parse(&self, lexer: &mut dyn Lexer, res: &mut Vec<Box<dyn AstTree>>) -> Result<(), String> {
        Ok(())
    }

    fn is_match(&self, lexer: &mut dyn Lexer) -> bool {
        self.leaf.is_match(lexer)
    }
}


/// 运算符优先级
/// precedence 中的 value 越大，说明优先级越高
///     如 + 运算的优先级低于 * 运算，故 value 应符合 * > +
/// 而 left_assoc 代表了 左结合 还是 右结合，代表了相同优先级时应该运算顺序是\[从左往右] 还是 \[从右往左]
///     如
///     = 赋值运算，右结合， 如一行多次赋值 a = b = c = 3,所有 = 号 具备相同优先级，最先执行的是最右边的赋值
///     + 加法运算，左结合
pub struct Precedence {
    value: usize,
    left_assoc: bool,
}

impl Precedence {
    pub fn left(value: usize) -> Precedence {
        Precedence { value, left_assoc: true }
    }
    pub fn right(value: usize) -> Precedence {
        Precedence { value, left_assoc: false }
    }
}

pub struct Operators {
    operators: HashMap<String, Rc<Precedence>>,
}

impl Operators {
    pub fn new() -> Self {
        Operators {
            operators: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, precedence: Precedence) -> &Self {
        self.operators.insert(name.to_string(), Rc::new(precedence));
        self
    }

    pub fn get(&self, name: &str) -> Option<Rc<Precedence>> {
        match self.operators.get(name) {
            None => { None }
            Some(rc_pre) => { Some(Rc::clone(rc_pre)) }
        }
    }

    pub fn rc(mut self) -> Rc<Self> {
       Rc::new(self)
    }
}

pub struct Expr {
    factor: Rc<Parser>,
    operators: Rc<Operators>,
    factory: Box<dyn AstFactory>,
}

impl Expr {
    pub fn new(factor: Rc<Parser>, operators: Rc<Operators>, factory: Box<dyn AstFactory>) -> Box<Self> {
        Box::new(Expr { factor, operators, factory })
    }

    // 此处为 peek，因为优先级问题，同一运算符可能被多次读取
    fn _next_operator(&self, lexer: &mut dyn Lexer) -> Option<Rc<Precedence>> {
        if let Some(box_token) = lexer.peek(0) {
            match box_token.value() {
                TokenValue::IDENTIFIER(id) => { self.operators.get(id) }
                _ => { None }
            }
        } else { None }
    }


    /// 右边是否是表达式
    /// 当右边优先级更高时，右边需要先计算，所以右边应该是表达式
    /// 当右边优先级小于等于左边时，符合从左往右，直接读区右边
    fn _right_is_expr(&self, precedence: &Precedence, next_precedence: &Precedence) -> bool {
        if next_precedence.left_assoc { precedence.value < next_precedence.value } else { precedence.value <= next_precedence.value }
    }

    /// 预读运算符，根据预读运算符判定是继续往下 shift 还是返回
    fn _do_shift(&self, lexer: &mut dyn Lexer, left: Box<dyn AstTree>, precedence: &Rc<Precedence>) -> Result<Box<dyn AstTree>, String> {
        let operator = AstLeaf::new(lexer.read().unwrap());
        let mut res = vec![left, operator];

        let mut right = self.factor.parse(lexer)?;
        while let Some(ref op) = self._next_operator(lexer) {
            let do_shift = self._right_is_expr(precedence.as_ref(), op.as_ref());
            right = if do_shift { self._do_shift(lexer, right, precedence)? } else { right }
        }
        res.push(right);
        Ok(self.factory.make(res))
    }
}

/// 定义 expr : factor { op factor }
/// 解析规则： 左结合 高优先级下沉，低、同优先级回退 ， 右结合，同，高优先级下沉，低优先级回退
///
/// 解析过程：
///     读取两个运算符，执行 shift 操作，读取右值并预读一个运算符
///         当前运算符优先级 > 预读运算符，说明当前运算符的左右值可以直接运算
///         若二者优先级相等，则根据结合性，左结合从左往右运算回退，右结合从右往左运算shift递进
///         若当前运算符优先级 < 预读运算符, 必须是先进行右边的运算，shift递进
///         例： a*b/c , op(*) = op(/), 优先级相同，且均为左结合， a*b 可以直接运算，解析结果为 AstTree((a*b)*c)
///             a*b+c , op(*) > op(+), 乘法优先级更高， a*b 可以直接运算
///             a = b = c + d , op(=) = op(=) ,优先级相同，且均为右结合，shift递进，解析结果为 AstTree(a=(b=(c+d)))
///         当前运算符优先级 < 预读运算符，说明当前运算符的左右值不可以直接运算，右值的运算要先进行
///         这种情况下继续执行 shift 操作递进，直到遇到 当前运算符优先级 >= 预读运算符
///         例： a+b/c , op(+) < op(/), 属于小于关系，对 a 的 + 操作依赖于 b/c，
///         shift操作会一直 while 循环并在循环中对右值做 shift 直到读取到 直到遇到 当前运算符优先级 >= 预读运算符
///         例： a+b/c*d+e
///             读取 a 和 + 后执行 shift(a,+) 操作
///             此时当前运算符是 op(+),在shift操作中
///                 op(+) < op(/),
///                     此时会继续递归一层 shift(b,/)，在内部 op(/) = op(*) 返回 AstTree(b/c)
///                 op(+) < op(*),
///                     此时会继续递归一层 shift((b/c),*)，在内部 op(*) > op(+) 返回 AstTree((b/c)*d)
///                 op(+) = op(=),返回  AstTree(a+((b/c)*d))
///             最后的解析结果就是  AstTree((a+((b/c)*d))+e)
///
/// 一个复杂表达式(a * 3 / b * 6 + 2 + d / e * 3)的解析过程如下：
/// ``` shell
/// parse right = a next_op = *
///     do_shift list = (a,*) right = 3 next_op = /  prec(* < /) false return (a,*,3)
/// parse right = a -> (a,*,3) , next_op = /
///     do_shift list = ((a,*,3),/) right = b next_op = *  prec(/ < *) false return ((a,*,3),/,b)
/// parse right = (a,*,3) -> ((a,*,3),/,b) , next_op = *
///     do_shift list = (((a,*,3),/,b),*) right = 6 next_op = +  prec(* < +) false return (((a,*,3),/,b),*,6)
/// parse right = ((a,*,3),/,b) -> (((a,*,3),/,b),*,6) , next_op = +
///     do_shift list = (((((a,*,3),/,b),*,6),+) right = 2 next_op = +  prec(+ < +) false return (((((a,*,3),/,b),*,6),+,2)
/// parse right = (((a,*,3),/,b),*,6) ->  (((((a,*,3),/,b),*,6),+,2) , next_op = +
///     do_shift list = ((((((a,*,3),/,b),*,6),+,2),+) right = d next_op = /  prec(+ < /) true and nextOP(/) not None is true do_shift
///         do_shift list = (d,/) right = e next_op = *  prec(/ < *) false return (d,/,e)
///     do_shift right = d -> (d,/,e) , next_op = * prec(+ < *) true and nextOP(*) not None is true do_shift
///         do_shift list = ((d,/,e),*) right = 3 next_op = None  return ((d,/,e),*,3)
///     do_shift right = (d,/,e) -> ((d,/,e),*,3) , next_op = None  return ((((((a,*,3),/,b),*,6),+,2),+,((d,/,e),*,3))
/// right =  ((((((a,*,3),/,b),*,6),+,2),+,((d,/,e),*,3))  next_op = None list.add(right)
/// ```

impl Element for Expr {
    fn parse(&self, lexer: &mut dyn Lexer, res: &mut Vec<Box<dyn AstTree>>) -> Result<(), String> {
        let mut right = self.factor.parse(lexer)?;
        while let Some(precedence) = self._next_operator(lexer) {
            right = self._do_shift(lexer, right, &precedence)?;
        }
        res.push(right);
        Ok(())
    }

    fn is_match(&self, lexer: &mut dyn Lexer) -> bool {
        self.factor.is_match(lexer)
    }
}