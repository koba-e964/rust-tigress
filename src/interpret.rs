use ast::{Expr, Dec, LValue, FunDec, TypeId, Value, Op};
use std::collections::HashMap;

enum LoopBreak {
    LoopBreak,
    Err(String),
}
type Env = (HashMap<String, usize> /* holds pointers */, HashMap<String, FunDec>, HashMap<String, ()> /* types */);
type VarPool = Vec<Value>; /* vector for variables */

fn get_var(name: &str, env: &Env, varpool: &mut VarPool) -> Result<Value, LoopBreak> {
    if let Some(&idx) = env.0.get(name) {
        Ok(varpool[idx].clone()) // TODO fix, because struct type must not be cloned here
    } else {
        Err(LoopBreak::Err("variable not found".to_string()))
    }
}
/* Even if name is already defined, this function creates another variable and hides the old one. */
fn define_var(name: &str, val: Value, env: &Env, varpool: &mut VarPool) -> Env {
    let mut cp_env = env.0.clone();
    let csize = varpool.len();
    varpool.push(val);
    cp_env.insert(name.to_string(), csize);
    (cp_env, env.1.clone(), env.2.clone())
}

fn update_var(name: &str, val: Value, env: &Env, varpool: &mut VarPool) -> Result<(), LoopBreak>{
    if let Some(&idx) = env.0.get(name) {
        varpool[idx] = val;
        Ok(())
    } else {
        Err(LoopBreak::Err("variable not found".to_string()))
    }
}

fn define_fun(name: &str, args: &Vec<(String, TypeId)>, opt_ty: &Option<TypeId>, body: &Expr, env: &Env) -> Env {
    let mut cp_funenv = env.1.clone();
    cp_funenv.insert(name.to_string(), (name.to_string(), args.clone(), opt_ty.clone(), body.clone()));
    (env.0.clone(), cp_funenv, env.2.clone())    
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
            Ok(())
        } else {
            Err(LoopBreak::Err("type_check failed".to_string()))
        }
    } else if ty == "string" {
        if let Value::VStr(_) = *val {
            Ok(())
        } else {
            Err(LoopBreak::Err("type_check failed".to_string()))
        }
    } else {
        panic!("type_check not implemented");
    }
}

fn f_sub(ast: &Expr, env: &Env, varpool: &mut VarPool, verbose: bool) -> Result<Value, LoopBreak> {
    if verbose {
        println!("env: {:?}, varpool: {:?}", env, varpool);
    }
    match *ast {
        Expr::Num(i) => Ok(Value::VNum(i)),
        Expr::Str(ref str) => Ok(Value::VStr(str.clone())),
        Expr::LVal(LValue::Id(ref x)) => get_var(x, env, varpool),
        Expr::LVal(_) => panic!("f_sub Expr::LVal"),
        Expr::Neg(ref e) =>
            match try!(f_sub(e, env, varpool, verbose)) {
                Value::VNum(i) => Ok(Value::VNum(-i)),
                _ => Err(LoopBreak::Err("Expr::Neg failed".to_string())),
            },

        Expr::OpNode(op, ref e1, ref e2) =>
            match op {
                Op::Add | Op::Sub | Op::Mul | Op::Div |
                Op::Lt | Op::Gt | Op::Le | Op::Ge => {
                    match (try!(f_sub(e1, env, varpool, verbose)),
                           try!(f_sub(e2, env, varpool, verbose))) {
                        (Value::VNum(i1), Value::VNum(i2)) =>
                            Ok(Value::VNum(arithmetic(op, i1, i2))),
                        _ => Err(LoopBreak::Err("arithmetic operation failed".to_string())),
                    }
                },
                Op::Eq | Op::Ne => {
                    let (v1, v2) = (try!(f_sub(e1, env, varpool, verbose)),
                                    try!(f_sub(e2, env, varpool, verbose)));
                    let res = (op == Op::Ne) ^ (v1 == v2);
                    Ok(Value::VNum(if res { 1 } else { 0 }))
                },
                Op::Or => {
                    let v1 = try!(f_sub(e1, env, varpool, verbose));
                    match v1 {
                        Value::VNum(0) => f_sub(e2, env, varpool, verbose),
                        Value::VNum(_) => Ok(v1),
                        _ => Err(LoopBreak::Err("type error in Op::Or".to_string())),
                    }
                },
                Op::And => {
                    let v1 = try!(f_sub(e1, env, varpool, verbose));
                    match v1 {
                        Value::VNum(0) => Ok(v1),
                        Value::VNum(_) => f_sub(e2, env, varpool, verbose),
                        _ => Err(LoopBreak::Err("type error in Op::And".to_string())),
                    }
                },
            },
        Expr::IfNode(ref cond, ref e_true, ref e_false) => 
            match try!(f_sub(cond, env, varpool, verbose)) {
                Value::VNum(0) => f_sub(e_false, env, varpool, verbose),
                Value::VNum(_) => f_sub(e_true, env, varpool, verbose),
                _ => Err(LoopBreak::Err("Condition of if has to be an integer.".to_string())),
            },
        Expr::Nil => Ok(Value::VNil),
        Expr::LAsgn(LValue::Id(ref name), ref e) => {
            try!(update_var(name, try!(f_sub(e, env, varpool, verbose)),
                            env, varpool));
            Ok(Value::VNil)
        },
        Expr::LAsgn(ref lval, ref e) => panic!("f_sub Expr::LAsgn"),
        Expr::Seq(ref es) => {
            let mut val = Value::VNil;
            for e in es {
                val = try!(f_sub(e, env, varpool, verbose));
            }
            Ok(val)
        },
        Expr::Let(ref decs, ref e2) => {
            let mut cp_env = env.clone();
            for dec in decs {
                match *dec {
                    Dec::Var(ref name, ref opt_ty, ref e) => {
                        let val = try!(f_sub(e, env, varpool, verbose));
                        // type-check
                        if let Some(ref ty) = *opt_ty {
                            try!(type_check(&val, ty));
                        }
                        cp_env = define_var(name, val, &cp_env, varpool);
                    }
                    Dec::Fun(ref name, ref args, ref opt_ty, ref body) => {
                        cp_env = define_fun(name, args, opt_ty, body, &cp_env);
                    }
                    _ => panic!("f_sub Expr::Let not supported")
                }
            }
            f_sub(e2, &cp_env, varpool, verbose)
        },
        Expr::For(ref var, ref st, ref en, ref body) => {
            if let Value::VNum(st_val) = try!(f_sub(st, env, varpool, verbose)) {
                if let Value::VNum(en_val) = try!(f_sub(en, env, varpool, verbose)) {
                    let cp_env = define_var(var, Value::VNum(st_val), env, varpool);
                    for i in st_val .. (en_val + 1) {
                        try!(update_var(var, Value::VNum(i), &cp_env, varpool));
                        let result = f_sub(body, &cp_env, varpool, verbose);
                        // TODO check if it is no result
                        if let Err(LoopBreak::LoopBreak) = result {
                            break;
                        }
                    }
                    return Ok(Value::VNoResult);
                }
            }
            panic!("f_sub Expr::For");
        },
        Expr::Do(ref cond, ref body) => {
            loop {
                if let Value::VNum(cval) = try!(f_sub(cond, env, varpool, verbose)) {
                    if cval == 0 {
                        break;
                    }
                    let result = f_sub(body, env, varpool, verbose);
                    // TODO check if it is no result
                    if let Err(LoopBreak::LoopBreak) = result {
                        break;
                    }
                }
            }
            return Ok(Value::VNoResult);
        },
        Expr::FunApp(ref f, ref es) => {
            // evaluate arguments from left to right
            match env.1.get(f) {
                Some(&(_, ref params, ref opt_retty, ref body)) => {
                    let n = es.len();
                    let mut args = vec![Value::VNum(0); n];
                    if params.len() != n {
                        return Err(LoopBreak::Err("wrong number of argument(s)".to_string()));
                    }
                    for i in 0 .. n {
                        args[i] = try!(f_sub(&es[i], env, varpool, verbose));
                        try!(type_check(&args[i], &params[i].1));
                    }
                    let mut cp_env = env.clone();
                    for i in 0 .. n {
                        cp_env = define_var(&params[i].0, args[i].clone(), &cp_env, varpool);
                    }
                    // TODO env handling
                    let result = try!(f_sub(&body, &cp_env, varpool, verbose));
                    if let Some(ref retty) = *opt_retty {
                        try!(type_check(&result, retty));
                    }
                    Ok(result)
                },
                None => Err(LoopBreak::Err("function not found in FunApp".to_string())),
            }
        },
        Expr::NewStruct(ref tyname, ref fields) =>
            Err(LoopBreak::Err("f_sub Expr::NewStruct".to_string())),
        Expr::NewArray(ref tyname, ref e, ref cnt) =>
            Err(LoopBreak::Err("f_sub Expr::NewArray".to_string())),
        Expr::Break => Err(LoopBreak::LoopBreak),
    }
}

pub fn f(ast: &Expr, verbose: bool) -> Value {
    match f_sub(ast, &(HashMap::new(), HashMap::new(), HashMap::new()), &mut Vec::new(), verbose) {
        Ok(result) => result,
        Err(LoopBreak::LoopBreak) => panic!("break outside loop was detected"),
        Err(LoopBreak::Err(str)) => panic!("interpret::f: {}", str),
    }
}

#[cfg(test)]
mod tests {
    use parse;
    use interpret;
    use ast::{Value};
    fn check(expr: &str, val: Value) {
        let ast = parse::parse(expr);
        assert_eq!(interpret::f(&ast, false), val);
    }
    #[test]
    fn letex_test() {
        check("let var x := 4 in x + x end", Value::VNum(8));
        check("let var x: int := 4 in x + x end",  Value::VNum(8));
        check("let var x := 4 in let var x := 3 in x + x end end",
              Value::VNum(6));
        check("let var x := 4 in (let var x := 3 in x end) + x end",
              Value::VNum(7));
    }
    #[test]
    fn comp_test() {
        check("2 < 5", Value::VNum(1));
        check("4 < 1", Value::VNum(0));
        check("2 = 2", Value::VNum(1));
        check("2 = 5", Value::VNum(0));
        check("2 <> 4", Value::VNum(1));
        check("2 <> 2", Value::VNum(0));
    }
    #[test]
    fn logic_test() {
        check("2 & 3", Value::VNum(3));
        check("0 & 3", Value::VNum(0));
        check("2 | 152", Value::VNum(2));
        check("0 | 155", Value::VNum(155));
    }
}
