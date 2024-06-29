use super::scope::{Scopes, Value};
use crate::compiler::ast::{self, Expr, OpCode};

pub fn run_ast_code(scopes: &Scopes, stats: &ast::Stats) -> (Value, bool) {
    let mut last_value = Value::None;

    for stat in stats {
        match stat {
            ast::Stat::Expr(expr) => last_value = eval_expr(scopes, expr),
            ast::Stat::VarDef(ast::VariableDefine { name, value }) => {
                scopes.add_variable(name.0.to_owned(), eval_expr(scopes, value));
                last_value = Value::None;
            }
            ast::Stat::VarMod(ast::VariableModify { name, op: _, value }) => {
                let val = eval_expr(scopes, &value);
                scopes.modify_variable(&name.0, val);
                last_value = Value::None;
            }
            ast::Stat::IfBlock(ast::IfBlock {
                cond,
                then,
                otherwise,
            }) => {
                let cond = eval_expr(scopes, &cond).to_bool();
                let subscope = scopes.new_sub();
                let (val, bk) = if cond {
                    run_ast_code(&subscope, then)
                } else {
                    run_ast_code(&subscope, otherwise)
                };
                if bk {
                    return (val, bk);
                }
                last_value = val;
            }
            ast::Stat::LoopBlock(ast::LoopBlock(body)) => {
                let subscope = scopes.new_sub();
                loop {
                    let (val, bk) = run_ast_code(&subscope, body);
                    if bk {
                        last_value = val;
                        break;
                    }
                }
            }
            ast::Stat::BreakStat => return (last_value, true),
        }
    }

    (last_value, false)
}

fn eval_expr(scopes: &Scopes, expr: &Expr) -> Value {
    match expr {
        Expr::Value(ast::Value::Identify(id)) => scopes.get_variable(&id.0).unwrap_or(Value::None),
        Expr::Value(ast::Value::String(v)) => Value::String(v.to_owned()),
        Expr::Value(ast::Value::Number(v)) => Value::Number(v.to_owned()),
        Expr::FuncCall(ast::FuncCall { name, params }) => {
            let mut vals = vec![];
            for p in params {
                vals.push(eval_expr(scopes, p));
            }
            scopes.call_function(&name.0, &vals).unwrap_or(Value::None)
        }
        Expr::Expr1(op, expr) => eval_op1(op, eval_expr(scopes, expr)),
        Expr::Expr2(expr1, op, expr2) => {
            eval_op2(op, eval_expr(scopes, expr1), eval_expr(scopes, expr2))
        }
    }
}

fn eval_op1(op: &OpCode, val: Value) -> Value {
    match op {
        OpCode::Add => Value::Number(val.to_number()),
        OpCode::Sub => Value::Number(-val.to_number()),
        OpCode::Not => Value::from_bool(!val.to_bool()),

        _ => Value::None,
    }
}

fn eval_op2(op: &OpCode, v1: Value, v2: Value) -> Value {
    match op {
        OpCode::Add => {
            if matches!(v1, Value::String(_)) {
                Value::String(v1.to_string().into_owned() + v2.to_string().as_ref())
            } else {
                Value::Number(v1.to_number() + v2.to_number())
            }
        }
        OpCode::Sub => Value::Number(v1.to_number() - v2.to_number()),
        OpCode::Mul => Value::Number(v1.to_number() * v2.to_number()),
        OpCode::Div => Value::Number(v1.to_number() / v2.to_number()),
        OpCode::EqEq => Value::from_bool(v1 == v2),
        OpCode::NotEq => Value::from_bool(v1 != v2),
        OpCode::Lt => Value::from_bool(v1.to_number() < v2.to_number()),
        OpCode::Gt => Value::from_bool(v1.to_number() > v2.to_number()),
        OpCode::Le => Value::from_bool(v1.to_number() <= v2.to_number()),
        OpCode::Ge => Value::from_bool(v1.to_number() >= v2.to_number()),
        OpCode::OrOr => Value::from_bool(v1.to_bool() || v2.to_bool()),
        OpCode::AndAnd => Value::from_bool(v1.to_bool() && v2.to_bool()),

        _ => Value::None,
    }
}
