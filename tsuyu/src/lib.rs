// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{fs::File, io::Write, process::Command, rc::Rc};
use tempfile::TempDir;

use tsuyu_ast::{self as ast};
use tsuyu_error::ComposedResult;
use tsuyu_lexer::tokenize;
use tsuyu_parser::parse;
use tsuyu_source::Source;
use tsuyuir as ir;
use tsuyuir::IR;

pub fn compile(source: Source) -> ComposedResult<String> {
    let s = Rc::new(source);
    let file = parse(tokenize(&s))?;

    let mut ir = IR::new();
    let main = ir.create_function("main").unwrap();
    for stmt in file.stmts {
        match stmt {
            ast::Stmt::Expr(expr) => main.body.push(ir::Stmt::Dump(match expr {
                ast::Expr::IntLiteral(n) => ir::Expr::Immediate(tsuyuir::Value::Int(n)),
            })),
        }
    }
    main.body
        .push(ir::Stmt::Return(ir::Expr::Immediate(tsuyuir::Value::Int(
            0,
        ))));

    Ok(tsuyuir_codegen::x86_64::compile(&ir))
}

pub fn run(source: Source) -> ComposedResult<std::process::Output> {
    let asm = compile(source)?;

    let tempdir = TempDir::new().expect("failed to create tempdir");
    let asm_path = tempdir.path().join("a.S");
    let bin_path = tempdir.path().join("a.out");

    let mut asm_file = File::create(&asm_path).expect("failed to create asm file");
    asm_file
        .write_all(asm.as_bytes())
        .expect("failed to write asm");

    Command::new("cc")
        .args(["-o", bin_path.to_str().unwrap(), asm_path.to_str().unwrap()])
        .status()
        .expect("failed to wait on child")
        .success()
        .then(|| ())
        .expect("assemble error");
    Ok(Command::new(bin_path)
        .output()
        .expect("failed to execute compiled binary"))
}
