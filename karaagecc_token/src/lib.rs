// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use karaagecc_source::Loc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    IntLiteral(i64),
    Error(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub loc: Loc,
}

impl TokenKind {
    pub fn into_token(self, loc: Loc) -> Token {
        Token { kind: self, loc }
    }
}

impl Token {
    pub fn new(kind: TokenKind, loc: Loc) -> Self {
        Self { kind, loc }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use karaage_asserts::*;
    fn_test_data_traits!(Token);

    #[test]
    fn test_new() {
        assert_eq!(
            Token::new(TokenKind::IntLiteral(42), Loc::head()),
            Token {
                kind: TokenKind::IntLiteral(42),
                loc: Loc::head(),
            }
        )
    }
}
