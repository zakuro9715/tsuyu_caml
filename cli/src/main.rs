// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate karaagecc;

use std::io::{self, Write};

#[cfg(not(tarpaulin_include))]
fn main() {
    match karaagecc::run("42") {
        Ok(output) => {
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
            std::process::exit(output.status.code().unwrap());
        }
        Err(e) => eprintln!("{}", e),
    }
}
