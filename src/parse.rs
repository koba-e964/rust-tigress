use ast::{Expr, FunDec};


/*
 * Example: ([(1, +), (2, -)], 4) ==> (1 + 2) - 4
 * Note that va.0 is in reverse order.
 */
fn vecast_to_ast<T, F>(va: (Vec<(Expr, T)>, Expr), fold: F) -> Expr
where F: Fn(T, Expr, Expr) -> Expr, T: Copy {
    let (mut x, y) = va;
    if x.len() == 0 {
        return y;
    }
    x.reverse();
    let mut ast = x[0].0.clone();
    for i in 0 .. x.len(){
        ast = fold(x[i].1, ast, if i == x.len() - 1 { y.clone() } else { x[i + 1].0.clone() });
    }
    ast
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
