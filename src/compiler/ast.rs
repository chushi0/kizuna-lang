use serde::{Deserialize, Serialize};

pub type Script = Vec<File>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Identify(pub String);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OpCode {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Not,
    EqEq,
    NotEq,
    Lt,
    Gt,
    Le,
    Ge,
    OrOr,
    AndAnd,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Expr {
    Value(Value),
    FuncCall(FuncCall),
    Expr1(OpCode, Box<Expr>),
    Expr2(Box<Expr>, OpCode, Box<Expr>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Stat {
    Expr(Box<Expr>),
    VarDef(VariableDefine),
    VarMod(VariableModify),
    IfBlock(IfBlock),
    LoopBlock(LoopBlock),
    BreakStat,
}

pub type Stats = Vec<Stat>;

#[derive(Debug, Serialize, Deserialize)]
pub struct IfBlock {
    pub cond: Box<Expr>,
    pub then: Stats,
    pub otherwise: Stats,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoopBlock(pub Stats);

#[derive(Debug, Serialize, Deserialize)]
pub enum File {
    Stat(Stat),
    FuncDef(FuncDef),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FuncDef {
    pub name: Identify,
    pub params: Vec<Identify>,
    pub body: Stats,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Value {
    Identify(Identify),
    String(String),
    Number(f64),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FuncCall {
    pub name: Identify,
    pub params: Vec<Box<Expr>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VariableDefine {
    pub name: Identify,
    pub value: Box<Expr>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VariableModify {
    pub name: Identify,
    pub op: OpCode,
    pub value: Box<Expr>,
}
