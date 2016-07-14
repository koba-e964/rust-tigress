// Expr
#[derive(PartialEq, Clone, Debug)]
pub enum Expr {
    Num(i64),
    Str(String),
    Var(String),
    LVal(LValue),
    Neg(Box<Expr>),
    OpNode(Op, Box<Expr>, Box<Expr>),
    IfNode(Box<Expr>, Box<Expr>, Box<Expr>), // If the first evaluates to non-zero, return the second. Otherwise, return the third.
    Nil,
    LAsgn(LValue, Box<Expr>),
    LetEx(String, Box<Expr>, Box<Expr>),
    FunApp(String, Vec<Expr>),
}

#[derive(PartialEq, Clone, Debug)]
pub enum LValue {
    Id(String),
    Mem(Box<LValue>, String),
    Idx(Box<LValue>, Box<Expr>),
}

pub type FunDec = (String, Vec<(String, Type)>, Type, Expr);

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Op {
    Add, Sub,
    Mul, Div,
    Eq, Ne, Lt, Gt, Le, Ge,
    Or, And,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    VNum(i64),
    VStr(String),
}

/*
 * Copy trait cannot be implemented because we might add some recursive constructors.
 */
#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    Int,
    Str,
}

#[derive(PartialEq, Clone, Debug)]
pub enum TypedExpr {
    Num(i64),
    Str(String),
    Var(String, Type),
    OpNode(Op, Type, Box<TypedExpr>, Box<TypedExpr>),
    IfNode(Box<TypedExpr>, Type, Box<TypedExpr>, Box<TypedExpr>),
    LetEx(String, Type, Box<TypedExpr>, Box<TypedExpr>),
    FunApp(String, Vec<Type>, Type, Vec<TypedExpr>), // name, argtype, rettype, arg
}

pub type TypedFunDec = (String, Vec<(String, Type)>, Type, TypedExpr);

pub fn ty_of_ast(tast: &TypedExpr) -> Type {
    match *tast {
        TypedExpr::Num(_) => Type::Int,
        TypedExpr::Str(_) => Type::Str,
        TypedExpr::Var(_, ref ty) => ty.clone(),
        TypedExpr::OpNode(_, ref ty, _, _) => ty.clone(),
        TypedExpr::IfNode(_, ref ty, _, _) => ty.clone(),
        TypedExpr::LetEx(_, ref ty, _, _) => ty.clone(),
        TypedExpr::FunApp(_, _, ref ty, _) => ty.clone(),
    }
}
