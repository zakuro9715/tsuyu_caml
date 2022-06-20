// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use tsuyu_ast::{expr, Expr, File, Stmt};
use tsuyu_error::{error, ComposedResult, Error, Result};
use tsuyu_source::Loc;
use tsuyu_token::{Token, TokenKind, TokenReader};

pub struct Parser<'a> {
    reader: TokenReader<'a>,
    loc: Loc,
}

impl<'a> Parser<'a> {
    pub fn new(reader: TokenReader<'a>) -> Self {
        let mut reader = reader;
        let loc = Loc::head(reader.peek().and_then(|t| t.loc.source()));
        Self { reader, loc }
    }
}

pub fn parse(reader: TokenReader<'_>) -> ComposedResult<File> {
    Parser::new(reader).parse()
}

macro_rules! expect_token_kind {
    (
        $tok:expr => {
            $(
                $kind:ident $( ( $( $field:ident ),* ) )? => $body:expr
            ),+ $(,)?
        }
    ) => {
        let __tok = $tok;
        match __tok.kind {
            $(
                TokenKind::$kind $( ( $( $field ),* ) )?
                    => Ok($body),
            )+
            _ => Err(error!("unexpected token", __tok.loc)),
        }
    }
}

impl<'a> Parser<'a> {
    fn peek(&mut self) -> Option<&Token> {
        self.reader.peek()
    }

    fn error(&self, msg: impl Into<String>) -> Error {
        error!(msg.into(), self.loc.clone())
    }

    fn read(&mut self) -> Result<Token> {
        self.reader
            .read()
            .map(|tok| {
                self.loc = tok.loc.clone();
                tok
            })
            .ok_or_else(|| self.error("unexpected EOF"))
    }

    fn eof(&mut self) -> bool {
        self.peek().is_none()
    }
}

impl<'a> Parser<'a> {
    pub fn parse(&mut self) -> ComposedResult<File> {
        let mut f = File::new(self.loc.source());
        let mut errors = Vec::new();
        while !self.eof() {
            match self.parse_stmt() {
                Ok(v) => {
                    f.stmts.push(v);
                }
                Err(err) => {
                    errors.push(err);
                }
            }
        }
        errors.is_empty().then(|| f).ok_or(errors)
    }

    fn parse_stmt(&mut self) -> Result<Stmt> {
        Ok(Stmt::Expr(self.parse_expr()?))
    }

    fn parse_expr(&mut self) -> Result<Expr> {
        self.parse_literal()
    }

    fn parse_literal(&mut self) -> Result<Expr> {
        expect_token_kind! {
            self.read()? => {
                IntLiteral(n) => expr!{ int(n) },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::rc::Rc;
    use tsuyu_asserts::assert_eq;
    use tsuyu_ast::{ast, File};
    use tsuyu_error::{error, Error};
    use tsuyu_lexer::tokenize;
    use tsuyu_source::{loc, Source};

    fn code(code: &str) -> Rc<Source> {
        Rc::new(Source::inline(code))
    }

    fn test(s: &Rc<Source>, expected: File) {
        assert_eq!(parse(tokenize(s)).unwrap(), expected);
    }

    fn test_error(s: &Rc<Source>, expected: Vec<Error>) {
        assert_eq!(parse(tokenize(s)).unwrap_err(), expected);
    }

    #[test]
    fn test_simple() {
        let s = code("42");
        test(&s, ast! { s => [ { int(42) } ] });

        let s = code("1 2\n3 4");
        test(
            &s,
            ast! { s => [
                { int(1) }, { int(2) },
                { int(3) }, { int(4) },
            ] },
        );
    }

    #[test]
    fn test_error_simple() {
        let s = code("x");
        test_error(
            &s,
            vec![error!("unexpected token", loc! {s => {0, 1;1, 1}})],
        );
    }
}
