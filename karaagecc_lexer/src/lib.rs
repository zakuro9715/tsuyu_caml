// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::iter::Peekable;

use karaagecc_chars::*;
use karaagecc_source::{Loc, Source};
use karaagecc_token::{Token, TokenKind, TokenReader};

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    loc: Loc,
    loc_head: Loc,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a Source) -> Self {
        Self {
            chars: source.code.chars().peekable(),
            loc: Loc::head(),
            loc_head: Loc::head(),
        }
    }

    fn peek_char(&mut self) -> Option<Char<'_>> {
        self.chars.peek().copied()
    }

    fn eof(&mut self) -> bool {
        self.peek_char().is_none()
    }

    fn consume(&mut self) -> Option<Char<'_>> {
        if self.eof() {
            panic!("Unexpected EOF")
        }

        let c = self.chars.next()?;

        self.loc_head.index += c.len();
        self.loc_head.column += 1;
        self.loc.len += c.len();

        if c == "\r" && self.peek_char().map_or(false, |c| c == "\n") {
            self.consume();
            self.loc_head.line -= 1; // rollback incremented line.
        }

        if is_newline(c) {
            self.loc_head.line += 1;
            self.loc_head.column = 1;
        }

        Some(c)
    }

    fn consume_if(&mut self, mut f: impl FnMut(Char<'_>) -> bool) -> Option<Char<'_>> {
        if f(self.peek_char()?) {
            Some(self.consume()?)
        } else {
            None
        }
    }

    fn consume_while(&mut self, mut f: impl FnMut(Char<'_>) -> bool) {
        while self.consume_if(|c| f(c)).is_some() {}
    }

    fn skip_whitespaces(&mut self) {
        self.consume_while(is_whitespace);
    }

    fn new_token(&self, kind: TokenKind) -> Token {
        Token {
            loc: self.loc,
            kind,
        }
    }
}

impl<'a> Lexer<'a> {
    fn read(&mut self) -> Option<Token> {
        self.skip_whitespaces();
        self.loc = self.loc_head;
        self.loc.len = 0;

        if self.eof() {
            return None;
        }

        if is_decimal_digit(self.peek_char()?) {
            Some(self.read_number())
        } else {
            self.consume_while(|c| !is_decimal_digit(c));
            Some(self.new_token(TokenKind::Error("err".to_string())))
        }
    }

    fn read_number(&mut self) -> Token {
        let mut value = 0;
        while let Some(c) = self.consume_if(is_decimal_digit) {
            value *= 10;
            value += c.char().to_digit(10).unwrap_or_else(|| unreachable!());
        }
        self.new_token(TokenKind::IntLiteral(value.into()))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.read()
    }
}

pub fn tokenize(s: &Source) -> TokenReader<'_> {
    TokenReader::new(Lexer::new(s))
}

#[cfg(test)]
mod tests {
    use crate::*;
    use karaage_asserts::*;
    use karaagecc_source::{loc, Source};
    use karaagecc_token::token;

    fn test(code: &str, expected: impl Into<Vec<Token>>) {
        let s = Source::inline(code);
        assert_iter_eq!(tokenize(&s), expected.into());
    }

    #[test]
    fn test_newline() {
        test(
            "1\n2\r3\r\n4\n\r6\n\n8\r\r10\n\r\n\r\n13",
            [
                token!(1, loc! {0,1; 1,1}),
                token!(2, loc! {2,3; 2,1}),
                token!(3, loc! {4,5; 3,1}),
                token!(4, loc! {7,8; 4,1}),
                token!(6, loc! {10,11; 6,1}),
                token!(8, loc! {13,14; 8,1}),
                token!(10, loc! {16,18; 10,1}),
                token!(13, loc! {23,25; 13,1}),
            ],
        );
    }

    #[test]
    fn test_number() {
        test(
            "1 16 256 2048",
            [
                token!(1, loc! {0,1;1,1}),
                token!(16, loc! {2,4;1,3}),
                token!(256, loc! {5,8;1,6}),
                token!(2048, loc! {9,13;1,10}),
            ],
        );
    }
}
