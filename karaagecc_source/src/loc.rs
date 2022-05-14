// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{cmp, fmt};

#[derive(PartialEq, Eq, Clone)]
pub struct Loc {
    pub index: usize,
    pub len: usize,
    pub line: usize,
    pub column: usize,
}

impl Default for Loc {
    fn default() -> Self {
        Self {
            index: 0,
            len: 1,
            line: 1,
            column: 1,
        }
    }
}

impl PartialOrd for Loc {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ index: {index}, len: {len}, line: {line}, column: {column} }}",
            index = self.index,
            len = self.len,
            line = self.line,
            column = self.column,
        )
    }
}

impl fmt::Debug for Loc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Loc {}", self)
    }
}

impl Loc {
    pub fn head() -> Self {
        Self::default()
    }

    pub fn new(index: usize, len: usize, line: usize, column: usize) -> Self {
        Self {
            index,
            len,
            line,
            column,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_head() {
        assert_eq!(Loc::head(), Default::default());
    }

    #[test]
    fn test_new() {
        assert_eq!(
            Loc::new(1, 1, 2, 3),
            Loc {
                index: 1,
                len: 1,
                line: 2,
                column: 3
            }
        );
    }

    #[test]
    fn test_fmt() {
        assert_eq!(
            format!("{}", Loc::new(1, 2, 2, 3)),
            "{ index: 1, len: 2, line: 2, column: 3 }",
        );
        assert_eq!(
            format!("{:?}", Loc::new(0, 1, 1, 2)),
            "Loc { index: 0, len: 1, line: 1, column: 2 }",
        );
    }
}
