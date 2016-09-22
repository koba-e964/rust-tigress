use ast::{Expr, Dec, LValue, FunDec, Value, Op};
use std::collections::HashMap;

enum LoopBreak {
    LoopBreak,
}
type Env = (HashMap<String, usize> /* holds pointers */, HashMap<String, FunDec>, HashMap<String, ()> /* types */);
type VarPool = Vec<Value>; /* vector for variables */

fn get_var(name: &str, env: &Env, varpool: &mut VarPool) -> Result<Value, LoopBreak> {
    let &idx = env.0.get(name).expect("variable not found");
    Ok(varpool[idx].clone()) // TODO fix, because struct type must not be cloned here
}
/* Even if name is already defined, this function creates another variable and hides the old one. */
fn define_var(name: &str, val: Value, env: &Env, varpool: &mut VarPool) -> Env {
    let mut cp_env = env.0.clone();
    let csize = varpool.len();
    varpool.push(val);
    cp_env.insert(name.to_string(), csize);
    (cp_env, env.1.clone(), env.2.clone())
}

fn update_var(name: &str, val: Value, env: &Env, varpool: &mut VarPool) {
    let &idx = env.0.get(name).expect("variable not found");
    varpool[idx] = val;
}

fn arithmetic(op: Op, v1: i64, v2: i64) -> i64 {
    match op {
        Op::Add => v1 + v2,
        Op::Sub => v1 - v2,
        Op::Mul => v1 * v2,
        Op::Div => v1 / v2,
        Op::Lt => if v1 < v2 { 1 } else { 0 },
        Op::Gt => if v1 > v2 { 1 } else { 0 },
        Op::Le => if v1 <= v2 { 1 } else { 0 },
        Op::Ge => if v1 >= v2 { 1 } else { 0 },
        _ => panic!("internal error (>_<)"),
    }
}

fn type_check(val: &Value, ty: &str) -> Result<(), LoopBreak> {
    if ty == "int" {
        if let Value::VNum(_) = *val {
        } else {
            panic!("type_check failed");
        }
        Ok(())
    } else if ty == "string" {
        if let Value::VStr(_) = *val {
        } else {
            panic!("type_check failed");
        }
        Ok(())
    } else {
        panic!("type_check not implemented");
    }
}

fn f_sub(ast: &Expr, env: &Env, varpool: &mut VarPool) -> Result<Value, LoopBreak> {
    match *ast {
        Expr::Num(i) => Ok(Value::VNum(i)),
        Expr::Str(ref str) => Ok(Value::VStr(str.clone())),
        Expr::LVal(LValue::Id(ref x)) => get_var(x, env, varpool),
        Expr::LVal(_) => panic!("f_sub Expr::LVal"),
        Expr::Neg(ref e) =>
            match try!(f_sub(e, env, varpool)) {
                Value::VNum(i) => Ok(Value::VNum(-i)),
                _ => panic!("Expr::Neg failed"),
            },

        Expr::OpNode(op, ref e1, ref e2) =>
            match op {
                Op::Add | Op::Sub | Op::Mul | Op::Div |
                Op::Lt | Op::Gt | Op::Le | Op::Ge => {
                    match (try!(f_sub(e1, env, varpool)),
                           try!(f_sub(e2, env, varpool))) {
                        (Value::VNum(i1), Value::VNum(i2)) =>
                            Ok(Value::VNum(arithmetic(op, i1, i2))),
                        _ => panic!("arithmetic operation failed"),
                    }
                },
                Op::Eq | Op::Ne => {
                    let (v1, v2) = (try!(f_sub(e1, env, varpool)),
                                    try!(f_sub(e2, env, varpool)));
                    let res = (op == Op::Ne) ^ (v1 == v2);
                    Ok(Value::VNum(if res { 1 } else { 0 }))
                },
                Op::Or => {
                    let v1 = try!(f_sub(e1, env, varpool));
                    match v1 {
                        Value::VNum(0) => f_sub(e2, env, varpool),
                        Value::VNum(_) => Ok(v1),
                        _ => panic!("type error in Op::Or"),
                    }
                },
                Op::And => {
                    let v1 = try!(f_sub(e1, env, varpool));
                    match v1 {
                        Value::VNum(0) => Ok(v1),
                        Value::VNum(_) => f_sub(e2, env, varpool),
                        _ => panic!("type error in Op::And"),
                    }
                },
            },
        Expr::IfNode(ref cond, ref e_true, ref e_false) => 
            match try!(f_sub(cond, env, varpool)) {
                Value::VNum(0) => f_sub(e_false, env, varpool),
                Value::VNum(_) => f_sub(e_true, env, varpool),
                _ => panic!("Condition of if has to be an integer."),
            },
        Expr::Nil => Ok(Value::VNil),
        Expr::LAsgn(LValue::Id(ref name), ref e) => {
            update_var(name, try!(f_sub(e, env, varpool)), env, varpool);
            Ok(Value::VNil)
        },
        Expr::LAsgn(ref lval, ref e) => panic!("f_sub Expr::LAsgn"),
        Expr::Seq(ref es) => {
            let mut val = Value::VNil;
            for e in es {
                val = try!(f_sub(e, env, varpool));
            }
            Ok(val)
        },
        Expr::Let(ref decs, ref e2) => {
            let mut cp_env = env.clone();
            for dec in decs {
                match *dec {
                    Dec::Var(ref name, ref opt_ty, ref e) => {
                        let val = try!(f_sub(e, env, varpool));
                        // type-check
                        if let Some(ref ty) = *opt_ty {
                            try!(type_check(&val, ty));
                        }
                        cp_env = define_var(name, val, &cp_env, varpool);
                    }
                    Dec::Fun(ref name, ref args, ref opt_ty, ref body) => {
                        panic!("function definition not implemented")
                    }
                    _ => panic!("f_sub Expr::Let not supported")
                }
            }
            f_sub(e2, &cp_env, varpool)
        },
        Expr::For(ref var, ref st, ref en, ref body) => {
            if let Value::VNum(st_val) = try!(f_sub(st, env, varpool)) {
                if let Value::VNum(en_val) = try!(f_sub(en, env, varpool)) {
                    let cp_env = define_var(var, Value::VNum(st_val), env, varpool);
                    for i in st_val .. (en_val + 1) {
                        update_var(var, Value::VNum(i), &cp_env, varpool);
                        let result = f_sub(body, &cp_env, varpool);
                        // TODO check if it is no result
                        if let Err(LoopBreak::LoopBreak) = result {
                            break;
                        }
                    }
                    return Ok(Value::VNil);
                }
            }
            panic!("f_sub Expr::For");
        },
        Expr::Do(ref cond, ref body) => {
            loop {
                if let Value::VNum(cval) = try!(f_sub(cond, env, varpool)) {
                    if (cval == 0) {
                        break;
                    }
                    let result = f_sub(body, env, varpool);
                    // TODO check if it is no result
                    if let Err(LoopBreak::LoopBreak) = result {
                        break;
                    }
                }
            }
            return Ok(Value::VNil);
        },
        Expr::FunApp(ref f, ref es) => {
            // evaluate arguments from left to right
            let n = es.len();
            let mut args = vec![Value::VNum(0); n];
            for i in 0 .. n {
                args[i] = try!(f_sub(&es[i], env, varpool));
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
    match f_sub(ast, &(HashMap::new(), HashMap::new(), HashMap::new()), &mut Vec::new()) {
        Ok(result) => result,
        Err(_) => panic!("f err"),
    }
}

#[cfg(test)]
mod tests {
    use parse;
    use interpret;
    use ast::{Value};
    #[test]
    fn letex_test() {
        let ast1 = parse::parse("let var x := 4 in x + x end");
        assert_eq!(interpret::f(&ast1), Value::VNum(8));
        let ast1_typed = parse::parse("let var x: int := 4 in x + x end");
        assert_eq!(interpret::f(&ast1_typed), Value::VNum(8));
        let ast2 = parse::parse("let var x := 4 in let var x := 3 in x + x end end");
        assert_eq!(interpret::f(&ast2), Value::VNum(6));
        let ast3 = parse::parse("let var x := 4 in (let var x := 3 in x end) + x end");
        assert_eq!(interpret::f(&ast3), Value::VNum(7));
    }
    #[test]
    fn comp_test() {
        let lt1 = parse::parse("2 < 5");
        assert_eq!(interpret::f(&lt1), Value::VNum(1));
        let lt2 = parse::parse("4 < 1");
        assert_eq!(interpret::f(&lt2), Value::VNum(0));
        let eq1 = parse::parse("2 = 2");
        assert_eq!(interpret::f(&eq1), Value::VNum(1));
        let eq2 = parse::parse("2 = 5");
        assert_eq!(interpret::f(&eq2), Value::VNum(0));
        let neq1 = parse::parse("2 <> 4");
        assert_eq!(interpret::f(&neq1), Value::VNum(1));
        let neq2 = parse::parse("2 <> 2");
        assert_eq!(interpret::f(&neq2), Value::VNum(0));
    }
    #[test]
    fn logic_test() {
        let and1 = parse::parse("2 & 3");
        assert_eq!(interpret::f(&and1), Value::VNum(3));
        let and2 = parse::parse("0 & 3");
        assert_eq!(interpret::f(&and2), Value::VNum(0));
        let or1 = parse::parse("2 | 152");
        assert_eq!(interpret::f(&or1), Value::VNum(2));
        let or2 = parse::parse("0 | 155");
        assert_eq!(interpret::f(&or2), Value::VNum(155));
    }
}
