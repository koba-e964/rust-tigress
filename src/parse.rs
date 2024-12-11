use crate::ast::{Dec, Expr, Field, LValue, Op, Type, TypeField};

/*
 * Example: ([(1, +), (2, -)], 4) ==> (4 + 1) - 2
 */
pub fn fold_to_ast<T, F>(va: (Vec<(Expr, T)>, Expr), fold: F) -> Expr
where
    F: Fn(T, Expr, Expr) -> Expr,
    T: Copy,
{
    let (x, y) = va;
    let mut ast = y;
    for (e, t) in x {
        ast = fold(t, ast, e);
    }
    ast
}

pub fn fold_op(first: Expr, ops: Vec<(Expr, Op)>) -> Expr {
    fold_to_ast((ops, first), |t, e1, e2| {
        Expr::OpNode(t, Box::new(e1), Box::new(e2))
    })
}

peg::parser! {
    grammar tigress_grammar() for str  {
        pub rule top_expr() -> Expr
            = space()* e:expr() space()* { e }
        rule expr() -> Expr
            = "for" space()+ i:id() space()* ":=" space()* s:expr() space()+ "to" space()+ u:expr() space()+ "do" space()+ e:expr() { Expr::For(i, Box::new(s), Box::new(u), Box::new(e)) }
            / "while" space()+ c:expr() space()+ "do" space()+ e:expr() { Expr::Do(Box::new(c), Box::new(e)) }
            / expr0()
        rule expr0() -> Expr /* if */
            = "if" space()* c:expr() space()* "then" space()* e1:expr1() !(space()+ "else") { Expr::IfNode(Box::new(c), Box::new(e1), Box::new(Expr::Nil)) }
            / expr1()
        rule expr1() -> Expr /* ifelse */
            = "if" space()* c:expr() space()* "then" space()* e1:expr2() space()* "else" space()* e2:expr2() { Expr::IfNode(Box::new(c), Box::new(e1), Box::new(e2)) }
            / expr2()
        rule expr2() -> Expr /* := */
            = l:lvalue() space()* ":=" space()* e:expr3() { Expr::LAsgn(l, Box::new(e)) }
            / expr3()
        rule expr3() -> Expr /* "|" */
            = e:expr4() ls:(space()* "|" space()* e:expr4() { (e, Op::Or) })* { fold_op(e, ls) }
        rule expr4() -> Expr /* "&" */
            = e:expr5() ls:(space()* "&" space()* e:expr5() { (e, Op::And) })* { fold_op(e, ls) }
        rule expr5() -> Expr /* %nonassoc "=" "<>" "<" ">" "<=" ">=" */
            = e1:expr6() space()* op:op5() space()* e2:expr6() { Expr::OpNode(op, Box::new(e1), Box::new(e2)) }
            / expr6()
        rule op5() -> Op /* %nonassoc "=" "<>" "<" ">" "<=" ">=" */
            = "=" { Op::Eq }
            / "<>" { Op::Ne }
            / "<=" { Op::Le }
            / ">=" { Op::Ge }
            / "<" { Op::Lt }
            / ">" { Op::Gt }
        rule expr6() -> Expr /* %left "+" "-" */
            = e:expr7() ls:(space()* o:op6() space()* e:expr7() { (e, o) })* { fold_op(e, ls) }
            / expr7()
        rule op6() -> Op /* %left "+" "-" */
            = "+" { Op::Add }
            / "-" { Op::Sub }
        rule expr7() -> Expr /* %left "*" "/" */
            = e:expr8() ls:(space()* o:op7() space()* e:expr8() { (e, o) })* { fold_op(e, ls) }
            / expr8()
        rule op7() -> Op /* %left "*" "/" */
            = "*" { Op::Mul }
            / "/" { Op::Div }
        rule expr8() -> Expr /* %right unary_minus */
            = "-" space()* e:expr8() { Expr::Neg(Box::new(e)) }
            / primary()
        rule primary() -> Expr
            = "let" space()+ decs:declaration_list() space()+ "in" space()+ es:expr_seq_opt() space()+ "end" { Expr::Let(decs, Box::new(Expr::Seq(es))) }
            / string_constant()
            / integer_constant()
            / "nil" { Expr::Nil }
            / i:id() space()* "(" space()* args:expr_list_opt() space()* ")" { Expr::FunApp(i, args) }
            / ty:id() space()* "{" space()* ls:field_list_opt() space()* "}" { Expr::NewStruct(ty, ls) }
            / ty:id() space()* "[" space()* n:expr() space()* "]" space()* "of" space()* e:expr()
                { Expr::NewArray(ty, Box::new(n), Box::new(e)) }
            / "(" space()* e:expr_seq_opt() space()* ")" { Expr::Seq(e) }
            / "break" { Expr::Break }
            / l:lvalue() { Expr::LVal(l) }

        rule expr_seq_opt() -> Vec<Expr>
            = expr() ++ (space()* ";" space()*)
            / { Vec::new() }

        rule expr_list_opt() -> Vec<Expr>
            = expr() ++ (space()* "," space()*)
            / { Vec::new() }

        rule field_list_opt() -> Vec<Field>
            = (i:id() space()* "=" space()* e:expr() { (i, e) }) ++ (space()* "," space()*)
            / { Vec::new() }

        rule lvalue() -> LValue
            = i:id() ls:(space()* l:lvalue_suffix() { l })* { {
                let mut acc = LValue::Id(i);
                for v in ls {
                    acc = match v {
                        Ok(name) => LValue::Mem(Box::new(acc),name),
                        Err(expr) => LValue::Idx(Box::new(acc), Box::new(expr)),
                    };
                }
                acc
            } }
        rule lvalue_suffix() -> Result<String, Expr>
            = "[" space()* e:expr() space()* "]" { Err(e) }
            / "." space()* i:id() { Ok(i) }

        rule declaration_list() -> Vec<Dec>
            = declaration() ++ (space()*)
        rule declaration() -> Dec
            = "type" space()+ tyid:id() space()* "=" space()* ty:type_() { Dec::Type(tyid, ty) }
            / "var" space()+ i:id() space()* ty:(":" space()* ty:id() space()* { ty })? ":=" space()* e:expr() { Dec::Var(i, ty, e) }
            / "function" space()+ fname:id() space()* "(" space()* fields:type_fields_opt() space()* ")" space()* retty:(":" space()* t:id() space()* { t })? "=" space()* e:expr() { Dec::Fun(fname, fields, retty, e) }

        rule integer_constant() -> Expr
            = mstr:$(['0'..='9']+) { Expr::Num(mstr.parse().unwrap()) }
        rule string_constant() -> Expr
            = "\"" s:str_internal() "\"" { Expr::Str(s) }
        rule str_internal() -> String
            = mstr:$([^'\\' | '"']*) { mstr.to_string() }
        rule space() -> ()
            = " " / "\n" / "\r" / "\t" / "\x0c"
        rule id() -> String
            = !keyword() mstr:$(['a'..='z' | 'A'..='Z'] ['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { mstr.to_string() }
        rule keyword() -> ()
            = "let" / "nil" / "of" / "then" / "to" / "type" / "var" / "while"
        rule type_() -> Type
            = "array" space()+ "of" space()+ t:id() { Type::Array(t) }
            / "{" space()* ls:type_fields_opt() space()* "}" { Type::Field(ls) }
            / ty:id() { Type::Id(ty) }

        rule type_fields_opt() -> Vec<TypeField>
            = (i:id() space()* ":" space()* ty:id() { (i, ty) }) ++ (space()* "," space()*)
            / { Vec::new() }
    }
}

pub fn parse(s: &str) -> Expr {
    match tigress_grammar::top_expr(s) {
        Ok(ast) => ast,
        Err(err) => {
            println!("{:?} in parsing {}", err, s);
            panic!("{err}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Expr::*;
    #[test]
    fn parse_test() {
        assert_eq!(
            parse("4 -2"),
            Expr::OpNode(Op::Sub, Box::new(Expr::Num(4)), Box::new(Expr::Num(2)))
        );
    }
    #[test]
    fn dangling_if_test() {
        assert_eq!(
            parse("if 4 then if 5 then 3 else 2"),
            Expr::IfNode(
                Box::new(Expr::Num(4)),
                Box::new(Expr::IfNode(
                    Box::new(Expr::Num(5)),
                    Box::new(Expr::Num(3)),
                    Box::new(Expr::Num(2)),
                )),
                Box::new(Expr::Nil),
            )
        );
    }
    #[test]
    fn dangling_do_test() {
        assert_eq!(
            parse("while 1 do 2 + 3"),
            Expr::Do(
                Box::new(Expr::Num(1)),
                Box::new(Expr::OpNode(
                    Op::Add,
                    Box::new(Expr::Num(2)),
                    Box::new(Expr::Num(3)),
                ))
            )
        );
    }
    #[test]
    fn let_test() {
        use crate::ast::Type::*;
        assert_eq!(
            parse("let var x:= 4 var y:=3 in x + y end"),
            Let(
                vec![
                    Dec::Var("x".to_string(), None, Num(4)),
                    Dec::Var("y".to_string(), None, Num(3)),
                ],
                Box::new(Seq(vec![OpNode(
                    Op::Add,
                    Box::new(LVal(LValue::Id("x".to_string()))),
                    Box::new(LVal(LValue::Id("y".to_string()))),
                )]))
            )
        );
        assert_eq!(
            parse("let type i = int in 3 end"),
            Let(
                vec![Dec::Type("i".to_string(), Type::Id("int".to_string()))],
                Box::new(Seq(vec![Num(3)])),
            )
        );
        assert_eq!(
            parse("let type int_array = array of int var x := int_array [4] of 0 in x end"),
            Let(
                vec![
                    Dec::Type("int_array".to_string(), Array("int".to_string())),
                    Dec::Var(
                        "x".to_string(),
                        None,
                        NewArray("int_array".to_string(), Box::new(Num(4)), Box::new(Num(0))),
                    )
                ],
                Box::new(Seq(vec![LVal(LValue::Id("x".to_string()))])),
            )
        );
        assert_eq!(
            parse("let type web = {dat: int} in web {dat = 3} end"),
            Let(
                vec![Dec::Type(
                    "web".to_string(),
                    Type::Field(vec![("dat".to_string(), "int".to_string())]),
                )],
                Box::new(Seq(vec![NewStruct(
                    "web".to_string(),
                    vec![("dat".to_string(), Num(3))],
                )]))
            )
        );
        assert_eq!(
            parse("let function f(x: int): int = x in x end"),
            Let(
                vec![Dec::Fun(
                    "f".to_string(),
                    vec![("x".to_string(), "int".to_string())],
                    Some("int".to_string()),
                    LVal(LValue::Id("x".to_string())),
                )],
                Box::new(Seq(vec![LVal(LValue::Id("x".to_string()))]))
            )
        );
    }
    #[test]
    fn new_struct_test() {
        assert_eq!(
            parse("web {dat = 3 }"),
            Expr::NewStruct("web".to_string(), vec![("dat".to_string(), Expr::Num(3))])
        );
        assert_eq!(
            parse("dummy {}"),
            Expr::NewStruct("dummy".to_string(), Vec::new())
        );
    }
    #[test]
    fn lvalue_member_test() {
        use crate::ast::LValue::*;
        assert_eq!(
            parse("x.member.t"),
            LVal(Mem(
                Box::new(Mem(Box::new(Id("x".to_string())), "member".to_string())),
                "t".to_string(),
            ))
        );
        assert_eq!(
            parse("x[4].go"),
            LVal(Mem(
                Box::new(Idx(Box::new(Id("x".to_string())), Box::new(Num(4)))),
                "go".to_string(),
            ))
        );
    }
}
