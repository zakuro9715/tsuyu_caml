// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use karaagecc_source::Source;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct File<'a> {
    pub source: &'a Source,
    pub stmts: Vec<Stmt>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stmt {
    Expr(Expr),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    IntLiteral(i64),
}

impl<'a> File<'a> {
    pub fn new(source: &'a Source) -> Self {
        Self {
            source,
            stmts: Vec::new(),
        }
    }
}

#[cfg(test)]
mod file_tests {
    use super::*;

    #[test]
    fn test_new() {
        let s = &Source::inline("");
        let f = File::new(s);
        assert_eq!(f.source, s);
        assert!(f.stmts.is_empty());
    }
}
