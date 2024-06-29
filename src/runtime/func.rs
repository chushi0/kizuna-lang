use crate::compiler::ast;

use super::{
    interpreter,
    scope::{SafeScope, Scopes, Value},
};

pub enum Func {
    NativeFunc(Box<dyn Fn(SafeScope, &[Value]) -> Value>),
    CodeFunc(ast::FuncDef),
}

impl Func {
    pub fn exec_call(&self, global_scope: SafeScope, params: &[Value]) -> Value {
        match self {
            Func::NativeFunc(f) => f(global_scope, params),
            Func::CodeFunc(ast) => {
                let scope = Scopes(vec![global_scope]).new_sub();
                for (i, param) in ast.params.iter().enumerate() {
                    let name = param.0.clone();
                    let value = if params.len() > i {
                        params[i].clone()
                    } else {
                        Value::None
                    };
                    scope.add_variable(name, value);
                }

                interpreter::run_ast_code(&scope, &ast.body).0
            }
        }
    }
}
