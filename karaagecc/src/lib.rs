// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
use karaagecc_ast as ast;
use karaagecc_error::{Error, ErrorKind::Message, Result};
use karaagecc_source::{Loc, Source};
use karaagecc_token::TokenKind::{self, IntLiteral};
use karaageir as ir;
use karaageir::IR;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use tempfile::TempDir;

pub fn compile(source: impl AsRef<Source>) -> Result<String> {
    let source = source.as_ref();
    let code = &source.code;
    let token = code
        .to_string()
        .trim()
        .parse::<i64>()
        .map(IntLiteral)
        .unwrap_or_else(|_| TokenKind::Error(format!("parse error: {} is not number", code)))
        .into_token(Loc::head());
    let mut file = ast::File::new(source);
    file.stmts.push(match token.kind {
        TokenKind::IntLiteral(n) => Ok(ast::Stmt::Expr(ast::Expr::IntLiteral(n))),
        TokenKind::Error(message) => Err(Error::new(Message(message))),
    }?);

    let mut ir = IR::new();
    let main = ir.create_function("main").unwrap();
    for stmt in file.stmts {
        match stmt {
            ast::Stmt::Expr(expr) => main.body.push(ir::Stmt::Dump(match expr {
                ast::Expr::IntLiteral(n) => ir::Expr::Immediate(karaageir::Value::Int(n)),
            })),
        }
    }
    main.body.push(ir::Stmt::Return(ir::Expr::Immediate(
        karaageir::Value::Int(0),
    )));

    Ok(karaageir_codegen::x86_64::compile(&ir))
}

pub fn run(source: impl AsRef<Source>) -> Result<std::process::Output> {
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
