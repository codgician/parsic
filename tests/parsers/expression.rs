use naive_parsec::combinators::char::char;
use naive_parsec::combinators::satisty::satisfy;
use naive_parsec::combinators::applicative::*;
use naive_parsec::combinators::functor::*;
use naive_parsec::combinators::or::*;
use naive_parsec::combinators::and::*;

/*
*     expr      := const | binOpExpr | neg | zero
*     const     := int
*     binOpExpr := '(' expr ' ' binOp ' ' expr ')'
*     binOp     := '+' | '*'
*     neg       := '-' expr
*     zero      := 'z'
*/

enum BinOp {
    AddBO,
    MulBO
}

enum Expr {
    ConstE(i32),
    BinOpE(BinOp, Expr, Expr),
    NegE(Expr),
    ZeroE
}

#[test]
fn test_expression() {
    let zero = char('z').map(|_| ZeroE);
    let neg = and(char('-'), expr)
                .map(|(_, x)| NegE(x));
    let cnst = some(satisfy(|ch| ch.is_digit(10)));
    let bin_op = or(char('+'), char('-'))
                .map(|op| if ch == '+' { Expr::AddBO } else { Expr::MulBO });
    let bin_op_expr = and(char('('), 
                        and(expr, 
                            and(char(' '), 
                                and(bin_op, 
                                    and(char(' '), 
                                        and(expr, 
                                            char(')')
                                        ).map(|(e, _)| e)
                                    ).map(|(_, e)| e)
                                )
                            ).map(|(_, t)| t)
                        ).map(|(e1, (op, e2))| Expr::BinOp(op, e1, e2))
                    ).map(|(_, x)| x);
    let expr = or(cnst, or(bin_op_expr, or(neg, zero)));
}


