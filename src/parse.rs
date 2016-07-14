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
    use ast::Expr::*;
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
    #[test]
    fn let_test() {
        use ast::Type::*;
        assert_eq!(parse("let var x:= 4 var y:=3 in x + y end"),
                   Let(vec![Dec::Var("x".to_string(), None, Num(4)), Dec::Var("y".to_string(), None, Num(3))],
                       Box::new(Seq(vec![OpNode(Op::Add,
                                                Box::new(LVal(LValue::Id("x".to_string()))),
                                                Box::new(LVal(LValue::Id("y".to_string()))))]))));
        assert_eq!(parse("let type i = int in 3 end"),
                   Let(vec![Dec::Type("i".to_string(), Type::Id("int".to_string()))], Box::new(Seq(vec![Num(3)]))));
        assert_eq!(parse("let type int_array = array of int var x := int_array [4] of 0 in x end"),
                   Let(vec![Dec::Type("int_array".to_string(), Array("int".to_string())), Dec::Var("x".to_string(), None, NewArray("int_array".to_string(), Box::new(Num(4)), Box::new(Num(0))))],
                       Box::new(Seq(vec![LVal(LValue::Id("x".to_string()))]))));
        assert_eq!(parse("let type web = {dat: int} in web {dat = 3} end"),
                   Let(vec![Dec::Type("web".to_string(), Type::Field(vec![("dat".to_string(), "int".to_string())]))], Box::new(Seq(vec![NewStruct("web".to_string(), vec![("dat".to_string(), Num(3))])]))));
    }
    #[test]
    fn new_struct_test() {
        assert_eq!(parse("web {dat = 3 }"),
                   Expr::NewStruct("web".to_string(),
                                   vec![("dat".to_string(), Expr::Num(3))]));
        assert_eq!(parse("dummy {}"), Expr::NewStruct("dummy".to_string(), Vec::new()));
    }
}
