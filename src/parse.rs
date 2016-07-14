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
        assert_eq!(parse("4 -2"), Expr::OpNode(Op::Sub, Box::new(Expr::Num(4)), Box::new(Expr::Num(2))));
    }
    #[test]
    fn dangling_if_test() {
        assert_eq!(parse("if 4 then if 5 then 3 else 2"),
                   Expr::IfNode(Box::new(Expr::Num(4)),
                                Box::new(Expr::IfNode(Box::new(Expr::Num(5)), Box::new(Expr::Num(3)), Box::new(Expr::Num(2)))),
                                Box::new(Expr::Nil)));
    }
    #[test]
    fn dangling_do_test() {
        assert_eq!(parse("while 1 do 2 + 3"),
                   Expr::Do(Box::new(Expr::Num(1)),
                            Box::new(Expr::OpNode(Op::Add, Box::new(Expr::Num(2)), Box::new(Expr::Num(3))))));
    }
}
