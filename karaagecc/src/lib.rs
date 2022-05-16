// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate karaagecc_source;
extern crate karaagecc_token;
extern crate karaageir;
extern crate karaageir_codegen;
extern crate tempfile;

use karaagecc_source::{Loc, Source};
use karaagecc_token::TokenKind::{self, IntLiteral};
use karaageir::{Expr, Stmt, IR};
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use tempfile::TempDir;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Error {
    Message(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Message(msg) => write!(f, "{}", msg),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn compile(source: impl AsRef<Source>) -> Result<String> {
    let code = &source.as_ref().code;
    let token = code
        .trim()
        .parse::<i64>()
        .map(|i| IntLiteral(i))
        .unwrap_or_else(|_| TokenKind::Error(format!("parse error: {} is not number", code)))
        .into_token(Loc::head());
    let value = match token.kind {
        IntLiteral(i) => Ok(i),
        TokenKind::Error(message) => Err(Error::Message(message)),
    }?;

    let mut ir = IR::new();
    let main = ir.create_function("main").unwrap();
    main.body
        .push(Stmt::Dump(Expr::Immediate(karaageir::Value::Int(value))));
    main.body
        .push(Stmt::Return(Expr::Immediate(karaageir::Value::Int(0))));

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
