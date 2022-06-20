// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use tsuyuir::{Expr, Function, Stmt, Type, Value, IR};
use std::{collections::HashMap, fmt::Write};

pub fn compile(ir: &IR<'_>) -> String {
    let mut gen = Gen::default();
    gen.gen(ir)
}

#[derive(Default)]
struct Gen {
    out_head: String,
    out_lc: String,
    out: String,
    lc_str_count: usize,
    indent_size: usize,
}

impl Gen {
    fn gen(&mut self, ir: &IR<'_>) -> String {
        self.out_head = ".intel_syntax noprefix\n".to_string();
        self.functions(&ir.functions);

        let outs = [&self.out_head, &self.out_lc, &self.out];
        let mut ret = String::with_capacity(outs.iter().fold(0, |sum, item| sum + item.len()));
        for out in outs {
            ret.write_str(out).unwrap();
        }
        ret
    }

    fn write(&mut self, s: &str) {
        if self.out.ends_with('\n') {
            self.out.write_str(&"\t".repeat(self.indent_size)).unwrap();
        }
        self.out.write_str(s).unwrap();
    }

    fn writeln(&mut self, s: &str) {
        self.write(s);
        self.out.push('\n');
    }

    fn indent(&mut self) {
        self.indent_size += 1;
    }

    fn unindent(&mut self) {
        self.indent_size -= 1;
    }

    fn string_constant(&mut self, value: &str) {
        let label = format!(".LC.str.{}", self.lc_str_count);
        self.lc_str_count += 1;
        write!(
            &mut self.out_lc,
            concat!(
                ".section .rodata\n",
                "{label}:\n",
                "\t.string \"{value}\"\n",
            ),
            label = label,
            value = value.replace('\n', "\\n").replace('\"', "\\\""),
        )
        .unwrap();
        self.write(&format!("[rip + {}]", &label));
    }

    fn functions<'a>(&mut self, funcs: &'a HashMap<&str, Function>) {
        self.writeln(".section .text");
        for name in funcs.keys() {
            self.writeln(&format!(".globl {}", name));
        }
        for (name, func) in funcs {
            self.writeln(&format!("{}:", name));
            self.indent();

            self.writeln("push rbp");
            self.writeln("mov rbp, rsp");

            for stmt in &func.body {
                self.stmt(stmt);
            }

            self.unindent();
        }
    }

    fn stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Return(expr) => {
                self.expr(expr);
                self.writeln("leave");
                self.writeln("ret")
            }
            Stmt::Dump(expr) => {
                self.expr(expr);
                self.writeln("mov rsi, rax");
                self.write("lea rdi, ");
                self.string_constant(&format!(
                    "{}\n",
                    match expr.typ() {
                        Type::Int => "%lld",
                    }
                ));
                self.writeln("");
                self.writeln("call printf@PLT")
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
