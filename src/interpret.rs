use ast::{Expr, FunDec, Value, Op};
use std::collections::HashMap;

enum LoopBreak {
    LoopBreak,
}
type Env = (HashMap<String, Value>, HashMap<String, FunDec>, HashMap<String, ()> /* types */);

fn get_var(name: &str, env: &Env) -> Result<Value, LoopBreak> {
    Ok(env.0.get(name).expect("variable not found").clone())
}
fn update_var(name: &str, val: Value, env: &Env) -> Env {
    let mut cp_env = env.0.clone();
    cp_env.insert(name.to_string(), val);
    (cp_env, env.1.clone(), env.2.clone())
}

fn arithmetic(op: Op, v1: i64, v2: i64) -> i64 {
    match op {
        Op::Add => v1 + v2,
        Op::Sub => v1 - v2,
        Op::Mul => v1 * v2,
        Op::Div => v1 / v2,
        _ => panic!("www"),
    }
}

fn f_sub(ast: &Expr, env: &Env) -> Result<Value, LoopBreak> {
    match *ast {
        Expr::Num(i) => Ok(Value::VNum(i)),
        Expr::Str(ref str) => Ok(Value::VStr(str.clone())),
        Expr::Var(ref x) => get_var(x, env),
        Expr::LVal(ref str) => panic!("f_sub Expr::LVal"),
        Expr::Neg(ref e) =>
            match try!(f_sub(e, env)) {
                Value::VNum(i) => Ok(Value::VNum(-i)),
                _ => panic!("Expr::Neg failed"),
            },

        Expr::OpNode(op, ref e1, ref e2) =>
            if op == Op::Add || op == Op::Sub || op == Op::Mul || op == Op::Div {
                match (try!(f_sub(e1, env)), try!(f_sub(e2, env))) {
                    (Value::VNum(i1), Value::VNum(i2)) => Ok(Value::VNum(arithmetic(op, i1, i2))),
                    _ => panic!("+ failed"),
                }
            } else {
                panic!("f_sub Expr::OpNode comparison");
            },
        Expr::IfNode(ref cond, ref e_true, ref e_false) => 
            match try!(f_sub(cond, env)) {
                Value::VNum(0) => f_sub(e_false, env),
                Value::VNum(_) => f_sub(e_true, env),
                _ => panic!("Condition of if has to be an integer."),
            },
        Expr::Nil => Ok(Value::VNil),
        Expr::LAsgn(ref lval, ref e) => panic!("f_sub Expr::LAsgn"),
        Expr::Seq(ref es) => {
            for e in es {
                try!(f_sub(e, env));
            }
            Ok(Value::VNil)
        },
        Expr::Let(ref asgns, ref e2) => {
            panic!("f_sub for Expr::Let");
        },
        Expr::For(ref var, ref st, ref en, ref body) => {
            panic!("f_sub Expr::For");
        },
        Expr::Do(ref cond, ref body) => {
            panic!("f_sub Expr::Do");
        },
        Expr::FunApp(ref f, ref es) => {
            // evaluate arguments from left to right
            let n = es.len();
            let mut args = vec![Value::VNum(0); n];
            for i in 0 .. n {
                args[i] = try!(f_sub(&es[i], env));
            }
            let mut cp_env = env.clone();
            panic!("FunApp");
        },
        Expr::NewStruct(ref tyname, ref fields) => panic!("f_sub Expr::NewStruct"),
        Expr::NewArray(ref tyname, ref e, ref cnt) => panic!("f_sub Expr::NewArray"),
        Expr::Break => Err(LoopBreak::LoopBreak),
    }
}

pub fn f(ast: &Expr) -> Value {
    match f_sub(ast, &(HashMap::new(), HashMap::new(), HashMap::new())) {
        Ok(result) => result,
        Err(_) => panic!("f err"),
    }
}

#[cfg(test)]
mod tests {
    use parse;
    use interpret;
    use ast::{Expr, Op, Value};
    #[test]
    fn operations_test() {
        let ast1 = Expr::OpNode(Op::Sub, Box::new(Expr::Num(7)), Box::new(Expr::Num(4)));
        assert_eq!(interpret::f(&Vec::new(), &ast1), Value::VNum(3));
        let ast2 = Expr::OpNode(Op::Div, Box::new(Expr::Num(20)), Box::new(Expr::Num(4)));
        assert_eq!(interpret::f(&Vec::new(), &ast2), Value::VNum(5));
    }
    #[test]
    fn letex_test() {
        let ast1 = parse::parse("let x = 4 in x + x");
        assert_eq!(interpret::f(&ast1.0, &ast1.1), Value::VNum(8));
        let ast2 = parse::parse("let x = 4 in let x = 3 in x + x");
        assert_eq!(interpret::f(&ast2.0, &ast2.1), Value::VNum(6));
        let ast3 = parse::parse("let x = 4 in (let x = 3 in x) + x");
        assert_eq!(interpret::f(&ast3.0, &ast3.1), Value::VNum(7));
    }
}
