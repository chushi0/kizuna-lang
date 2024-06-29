use crate::compiler::ast;

use super::{
    interpreter,
    scope::{SafeScope, Scopes},
};

pub struct VM {
    global_scope: SafeScope,
}

impl VM {
    pub fn new() -> Self {
        VM {
            global_scope: SafeScope::new(),
        }
    }

    pub fn global_scope(&self) -> SafeScope {
        self.global_scope.clone()
    }

    pub fn submit_script(&self, script: ast::Script) {
        let scopes = Scopes(vec![self.global_scope.clone()]);
        for file in script {
            match file {
                ast::File::Stat(stat) => {
                    interpreter::run_ast_code(&scopes, &vec![stat]);
                }
                ast::File::FuncDef(func) => scopes.add_code_function(func),
            }
        }
    }
}
