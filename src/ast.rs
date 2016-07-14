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
    Seq(Vec<Expr>),
    Let(Vec<Dec>, Box<Expr>),
    For(String, Box<Expr>, Box<Expr>, Box<Expr>),
    Do(Box<Expr>, Box<Expr>),
    FunApp(String, Vec<Expr>),
    NewStruct(String, Vec<Field>),
    NewArray(String, Box<Expr>, Box<Expr>),
    Break,
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
pub enum Dec {
    Type(String, Type),
    Var(String, Option<String>, Expr), // second is type-id
    Fun(String, Vec<TypeField>, Option<String>, Expr), // third is type-id
}

pub type Field = (String, Expr);
pub type TypeField = (String, String);

#[derive(PartialEq, Clone, Debug)]
pub enum Value {
    VNum(i64),
    VStr(String),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    Id(String),
    Field(Vec<TypeField>),
    Array(String),
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
        TypedExpr::Num(_) => Type::Id("int".to_string()),
        TypedExpr::Str(_) => Type::Id("str".to_string()),
        TypedExpr::Var(_, ref ty) => ty.clone(),
        TypedExpr::OpNode(_, ref ty, _, _) => ty.clone(),
        TypedExpr::IfNode(_, ref ty, _, _) => ty.clone(),
        TypedExpr::LetEx(_, ref ty, _, _) => ty.clone(),
        TypedExpr::FunApp(_, _, ref ty, _) => ty.clone(),
    }
}
