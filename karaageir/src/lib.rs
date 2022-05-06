use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
    Int(i64),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Immediate(Value),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Return(Expr),
}

#[derive(Debug, Clone, Default)]
pub struct Function {
    pub body: Vec<Stmt>,
}

impl Function {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Default)]
pub struct IR<'a> {
    pub functions: HashMap<&'a str, Function>,
}

impl<'a> IR<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_function(&mut self, name: &'a str) -> Option<&mut Function> {
        self.functions.insert(name, Function::new());
        self.functions.get_mut(name)
    }
}
