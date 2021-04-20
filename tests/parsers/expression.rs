use naive_parsec::combinators::*;

/*
*     expr      := const | binOpExpr | neg | zero
*     const     := int
*     binOpExpr := '(' expr ' ' binOp ' ' expr ')'
*     binOp     := '+' | '*'
*     neg       := '-' expr
*     zero      := 'z'
*/

type Int = i64;

enum BinOp {
    AddBO,
    MulBO
}

enum Expr {
    ConstE(Int),
    BinOpE(BinOp, Box<Expr>, Box<Expr>),
    NegE(Box<Expr>),
    ZeroE
}

fn parse_expression(inp: &str) -> Expr {
}

fn eval_expression(expr: Expr) -> Int {

}

#[test]
fn expression_test() {

}
