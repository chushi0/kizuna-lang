use std::{
    borrow::Cow,
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::compiler::ast;

use super::func::Func;

#[derive(Clone, PartialEq)]
pub enum Value {
    None,
    String(String),
    Number(f64),
}

impl Value {
    pub fn to_number(&self) -> f64 {
        match self {
            Value::None => 0.0,
            Value::String(s) => s.parse().unwrap_or(0.0),
            Value::Number(n) => *n,
        }
    }

    pub fn to_string(&self) -> Cow<'_, str> {
        match self {
            Value::None => Cow::Borrowed(""),
            Value::String(s) => Cow::Borrowed(s),
            Value::Number(n) => Cow::Owned(n.to_string()),
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            Value::None => false,
            Value::String(s) => !s.is_empty(),
            Value::Number(n) => *n > 0.0,
        }
    }

    pub fn from_bool(bl: bool) -> Value {
        if bl {
            Value::Number(1.0)
        } else {
            Value::Number(0.0)
        }
    }
}

pub struct Scope {
    variables: HashMap<String, Value>,
    functions: HashMap<String, Func>,
}

#[derive(Clone)]
pub struct SafeScope(pub Arc<RwLock<Scope>>);

pub struct Scopes(pub Vec<SafeScope>);

impl Scope {
    pub fn new() -> Scope {
        Scope {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn add_variable(&mut self, name: String, val: Value) {
        self.variables.insert(name, val);
    }

    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    pub fn get_variable_mut(&mut self, name: &str) -> Option<&mut Value> {
        self.variables.get_mut(name)
    }

    pub fn add_native_function<F>(&mut self, name: String, body: F)
    where
        F: Fn(SafeScope, &[Value]) -> Value + 'static,
    {
        self.functions
            .insert(name, Func::NativeFunc(Box::new(body)));
    }

    pub fn add_code_function(&mut self, ast: ast::FuncDef) {
        self.functions
            .insert(ast.name.0.clone(), Func::CodeFunc(ast));
    }
}

impl SafeScope {
    pub fn new() -> SafeScope {
        SafeScope(Arc::new(RwLock::new(Scope::new())))
    }
}

impl Scopes {
    pub fn add_variable(&self, name: String, val: Value) {
        self.0.first().map(|scope| {
            scope
                .0
                .write()
                .map(|mut scope| scope.add_variable(name, val))
        });
    }

    pub fn add_native_function<F>(&self, name: String, body: F)
    where
        F: Fn(SafeScope, &[Value]) -> Value + 'static,
    {
        self.0.first().map(|scope| {
            scope
                .0
                .write()
                .map(|mut scope| scope.add_native_function(name, body))
        });
    }

    pub fn add_code_function(&self, ast: ast::FuncDef) {
        self.0.first().map(|scope| {
            scope
                .0
                .write()
                .map(|mut scope| scope.add_code_function(ast))
        });
    }

    pub fn get_variable(&self, name: &str) -> Option<Value> {
        for scope in &self.0 {
            if let Some(val) = scope
                .0
                .write()
                .map(|scope| scope.get_variable(name).cloned())
                .ok()
                .flatten()
            {
                return Some(val);
            }
        }

        None
    }

    pub fn modify_variable(&self, name: &str, new_value: Value) {
        for scope in &self.0 {
            if scope
                .0
                .write()
                .map(|mut scope| {
                    scope.get_variable_mut(name).map(|var| {
                        *var = new_value.clone();
                        true
                    })
                })
                .ok()
                .flatten()
                .unwrap_or(false)
            {
                break;
            }
        }
    }

    pub fn call_function(&self, name: &str, params: &[Value]) -> Option<Value> {
        for scope in &self.0 {
            if let Some(val) = scope
                .0
                .read()
                .map(|scope| {
                    scope
                        .functions
                        .get(name)
                        .map(|func| func.exec_call(self.0.last().cloned().unwrap(), params))
                })
                .ok()
                .flatten()
            {
                return Some(val);
            }
        }

        None
    }

    pub fn new_sub(&self) -> Scopes {
        let mut sub = Scopes(vec![]);
        sub.0.push(SafeScope::new());

        for scope in &self.0 {
            sub.0.push(scope.clone());
        }

        sub
    }
}
