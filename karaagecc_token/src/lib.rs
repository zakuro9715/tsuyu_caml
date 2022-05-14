// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

extern crate karaagecc_source;
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
        Token{
            kind: self,
            loc,
        }
    }
}