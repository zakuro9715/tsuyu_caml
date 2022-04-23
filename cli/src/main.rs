/* main.rs
 * Copyright (C) 2022 zakuro <z@kuro.red>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.

 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

extern crate karaagecc_compiler;

use std::io::{self, Write};
use karaagecc_compiler as karaagecc;

#[cfg(not(tarpaulin_include))]
fn main() {
    match karaagecc::run() {
        Ok(output) => {
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
            std::process::exit(0);
        }
        Err(_) => panic!("err"),
    }
}
