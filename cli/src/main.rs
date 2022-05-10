// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate karaagecc;
extern crate karaagecc_source;

use std::io::{self, Write};
use clap::{Parser};
use karaagecc_source::Source;

#[derive(Parser)]
#[clap(
    name = "karaagecc",
    version,
    about = "Karaage C Compiler",
    help_template = "\
{name} - {about}

{usage-heading}
    {usage}
{all-args}"
)]
struct App {
}


#[cfg(not(tarpaulin_include))]
fn main() {
    let cli = App::parse();
    match karaagecc::run(Source::inline("42")) {
        Ok(output) => {
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
            std::process::exit(output.status.code().unwrap());
        }
        Err(e) => eprintln!("{}", e),
    }
}
