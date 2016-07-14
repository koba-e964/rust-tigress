use ast::{Expr, Op};


/*
 * Example: ([(1, +), (2, -)], 4) ==> (4 + 1) - 2
 */
pub fn fold_to_ast<T, F>(va: (Vec<(Expr, T)>, Expr), fold: F) -> Expr
where F: Fn(T, Expr, Expr) -> Expr, T: Copy {
    let (x, y) = va;
    let mut ast = y;
    for (e, t) in x {
        ast = fold(t, ast, e);
    }
    ast
}

pub fn fold_op(first: Expr, ops: Vec<(Expr, Op)>) -> Expr {
    fold_to_ast((ops, first), |t, e1, e2| Expr::OpNode(t, Box::new(e1), Box::new(e2)))
}

peg! tigress_grammar(include_str!("grammar.rustpeg"));


pub fn parse(s: &str) -> Expr {
    match tigress_grammar::top_expr(s) {
        Ok(ast) => ast,
        Err(err) => { println!("{:?}", err); panic!(err) }
    }
}

#[cfg(test)]
mod tests {
    use parse::*;
    use ast::*;
    #[test]
    fn parse_test() {
        assert_eq!(parse("4 -2").1, Expr::OpNode(Op::Sub, Box::new(Expr::Num(4)), Box::new(Expr::Num(2))));
        assert_eq!(parse("let x = 4 in x + y * 2").1,
            Expr::LetEx("x".to_string(), Box::new(Expr::Num(4)),
                       Box::new(Expr::OpNode(Op::Add, Box::new(Expr::Var("x".to_string())),
                                    Box::new(Expr::OpNode(Op::Mul, Box::new(Expr::Var("y".to_string())), Box::new(Expr::Num(2)))))))
        );
    }
}
