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

impl TokenKind {
    pub fn into_token(self, loc: Loc) -> Token {
        Token { kind: self, loc }
    }
}

macro_rules! impl_from_for_token_kind {
    ($variant:ident ( $t:ty )) => {
        impl From<$t> for TokenKind {
            fn from(v: $t) -> Self {
                TokenKind::$variant(v)
            }
        }
    };
    ($( $variant:ident ( $t:tt ) ),+ $(,)? ) => {
        $(
            impl_from_for_token_kind!{$variant ( $t )}
        )+
    };
}

impl_from_for_token_kind! {
    IntLiteral(i64),
}

#[cfg(test)]
mod token_kind_tests {
    use crate::{TokenKind::*, *};
    #[test]
    fn test_into_token() {
        let kind = IntLiteral(42);
        let loc = Loc::head();
        let expected = Token {
            kind: kind.clone(),
            loc: loc.clone(),
        };
        assert_eq!(kind.into_token(loc), expected);
    }

    #[test]
    fn test_token_kind_from() {
        assert_eq!(TokenKind::from(42), IntLiteral(42));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub loc: Loc,
}

impl Token {
    pub fn new(kind: TokenKind, loc: Loc) -> Self {
        Self { kind, loc }
    }
}

#[macro_export]
macro_rules! token {
    ($v:expr, $loc:expr) => {
        TokenKind::from($v).into_token($loc)
    };
}

#[cfg(test)]
mod token_tests {
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

    #[test]
    fn test_token_macro() {
        assert_eq!(
            token!(42, Loc::head()),
            Token::new(TokenKind::IntLiteral(42), Loc::head()),
        )
    }
}
