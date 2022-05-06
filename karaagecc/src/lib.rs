// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate karaageir;
extern crate karaageir_codegen;
extern crate tempfile;

use karaageir::{Expr, Stmt, IR};
use std::fs::File;
use std::io::Write;
use std::process::Command;
use tempfile::TempDir;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Error {
    Message(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn compile() -> Result<String> {
    let mut ir = IR::new();
    let main = ir.create_function("main").unwrap();
    main.body
        .push(Stmt::Return(Expr::Immediate(karaageir::Value::Int(42))));
    Ok(karaageir_codegen::x86_64::compile(&ir))
}

pub fn run() -> Result<std::process::Output> {
    let asm = compile()?;

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
