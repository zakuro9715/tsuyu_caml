// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::rc::Rc;

use tsuyu_utils::{clone_option_rc, define_with_params_and_init};
use tsuyu_source::Source;

define_with_params_and_init! {
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct File {
        source: Option<Rc<Source>>,
        pub stmts: Vec<Stmt>,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stmt {
    Expr(Expr),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    IntLiteral(i64),
}

impl File {
    pub fn new(s: Option<&Rc<Source>>) -> Self {
        Self {
            source: clone_option_rc(s),
            stmts: Vec::new(),
        }
    }

    pub fn source(&self) -> Option<&Rc<Source>> {
        self.source.as_ref()
    }
}

#[cfg(test)]
mod file_tests {
    use super::*;

    #[test]
    fn test_new() {
        let s = Rc::new(Source::inline(""));
        let f = File::new(Some(&s));
        assert_eq!(f.source, Some(s));
        assert!(f.stmts.is_empty());
    }
}

#[macro_export]
macro_rules! ast {
    (
        $source:expr => [
            $(
                { $( $stmt:tt )* }
            ),+ $(,)?
        ]
    ) => {
        $crate::File::init($crate::FileInitParams{
            source: Some(Rc::clone(&$source)),
            stmts: vec![
                $( $crate::stmt!($( $stmt )*) ),+
            ]
        })
    };
    (
        [
            $(
                { $( $stmt:tt )* }
            ),+ $(,)?
        ]
    ) => {
        $crate::File::init($crate::FileInitParams{
            source: None,
            stmts: vec![
                $( $crate::stmt!($( $stmt )*) ),+
            ]
        })
    };
}

#[macro_export]
macro_rules! stmt {
    ($( $input:tt )* ) => {
        $crate::Stmt::Expr($crate::expr!($( $input )*))
    };
}

#[macro_export]
macro_rules! expr {
    (int ( $expr:expr )) => {
        $crate::Expr::IntLiteral($expr)
    };
}

#[cfg(test)]
mod macro_tests {
    use std::rc::Rc;

    use super::ast;
    use crate as ast;
    use tsuyu_asserts::assert_eq;
    use tsuyu_source::Source;

    #[test]
    fn test_macro() {
        let s = Rc::new(Source::inline("1\n2"));
        assert_eq!(
            ast! {
                s => [
                    { int(1) },
                    { int(2) },
                ]
            },
            ast::File {
                source: Some(Rc::clone(&s)),
                stmts: vec![
                    ast::Stmt::Expr(ast::Expr::IntLiteral(1)),
                    ast::Stmt::Expr(ast::Expr::IntLiteral(2)),
                ],
            }
        );

        assert_eq!(
            ast! { [{ int(1) }, { int(2) }] },
            ast::File {
                source: None,
                stmts: vec![
                    ast::Stmt::Expr(ast::Expr::IntLiteral(1)),
                    ast::Stmt::Expr(ast::Expr::IntLiteral(2)),
                ],
            }
        );

        assert_eq!(stmt! { int(1) }, ast::Stmt::Expr(ast::Expr::IntLiteral(1)));
        assert_eq!(expr! { int(1) }, ast::Expr::IntLiteral(1));
    }
}
