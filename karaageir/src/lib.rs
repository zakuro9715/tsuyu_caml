use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type {
    Int,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
    Int(i64),
}

impl Value {
    pub fn typ(&self) -> Type {
        match self {
            Value::Int(_) => Type::Int,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expr {
    Immediate(Value),
}

impl Expr {
    pub fn typ(&self) -> Type {
        match self {
            Expr::Immediate(v) => v.typ(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Stmt {
    Dump(Expr),
    Return(Expr),
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Function {
    pub body: Vec<Stmt>,
}

impl Function {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
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

#[cfg(tests)]
mod tests {
    use crate::*;
    use karaage_asserts::*;

    fn_test_data_traits!(IR);
}
