extern crate karaagecc;

use std::io::{self, Write};

#[cfg(not(tarpaulin_include))]
fn main() {
    match karaagecc::run() {
        Ok(output) => {
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
            std::process::exit(output.status.code().unwrap());
        }
        Err(_) => panic!("err"),
    }
}
