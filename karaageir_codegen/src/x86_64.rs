// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate karaageir;
use karaageir::{Expr, Function, Stmt, Value, IR};
use std::{collections::HashMap, default, fmt::Write};

pub fn compile(ir: &IR) -> String {
    let mut gen = Gen::default();
    gen.gen(&ir);
    gen.out
}

#[derive(Default)]
struct Gen {
    out: String,
    indent_size: usize,
}

impl Gen {
    fn gen(&mut self, ir: &IR) {
        self.out = ".intel_syntax noprefix\n".to_string();
        self.functions(&ir.functions);
    }

    fn write(&mut self, s: &str) {
        if self.out.ends_with("\n") {
            self.out.write_str(&"\t".repeat(self.indent_size)).unwrap();
        }
        self.out.write_str(s).unwrap();
    }

    fn writeln(&mut self, s: &str) {
        self.write(s);
        self.out.push_str("\n");
    }

    fn indent(&mut self) {
        self.indent_size += 1;
    }

    fn unindent(&mut self) {
        self.indent_size -= 1;
    }

    fn functions<'a>(&mut self, funcs: &'a HashMap<&str, Function>) {
        for name in funcs.keys() {
            self.writeln(&format!(".globl {}", name));
        }
        for (name, func) in funcs {
            self.writeln(&format!("{}:", name));
            self.indent();
            for stmt in &func.body {
                self.stmt(stmt);
            }
            self.unindent();
        }
    }

    fn stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Return(expr) => {
                self.expr(&expr);
                self.writeln("ret")
            }
        }
    }

    fn expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Immediate(value) => match value {
                Value::Int(i) => self.writeln(&format!("mov rax, {}", i)),
            },
        }
    }
}
