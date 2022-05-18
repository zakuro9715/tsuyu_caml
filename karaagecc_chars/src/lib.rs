// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{fmt, hash::Hash};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Char<'a> {
    raw: &'a str,
}

impl<'a> Char<'a> {
    fn byte(&self) -> u8 {
        self.raw.bytes().next().expect("Char is broken")
    }

    pub fn char(&self) -> char {
        self.raw.chars().next().expect("Char is broken")
    }

    pub fn ascii(&self) -> Option<u8> {
        let b = self.byte();
        if b.is_ascii() {
            Some(b)
        } else {
            None
        }
    }
}

// --- traits ---

impl<'a> From<&'a str> for Char<'a> {
    fn from(raw: &'a str) -> Self {
        Self { raw }
    }
}

#[test]
fn test_from_str() {
    let c = '🐈';
    assert_eq!(Char::from(c.to_string().as_str()).char(), c)
}

impl<'a> fmt::Display for Char<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.raw.fmt(f)
    }
}

impl<'a> fmt::Debug for Char<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.raw.fmt(f)
    }
}

#[test]
fn test_fmtr() {
    let s = "🐈";
    let c = Char::from(s);
    assert_eq!(format!("{}", c), format!("{}", s));
    assert_eq!(format!("{:?}", c), format!("{:?}", s));
}

// --- traits end --

pub fn is_whitespace(c: Char<'_>) -> bool {
    c.char().is_whitespace()
}

pub fn is_octal_digit(c: Char<'_>) -> bool {
    (b'0'..b'8').contains(&c.byte())
}

pub fn is_decimal_digit(c: Char<'_>) -> bool {
    c.char().is_ascii_digit()
}

pub fn is_hex_digit(c: Char<'_>) -> bool {
    c.char().is_ascii_hexdigit()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_is_whitespace() {
        for c in [" ", "\t", "\n", "\r"] {
            assert!(is_whitespace(c.into()));
        }
        for c in ["a", "あ"] {
            assert!(!is_whitespace(c.into()));
        }
    }

    #[test]
    fn test_is_digit() {
        for c in ["0", "3", "7"] {
            assert!(is_decimal_digit(c.into()));
            assert!(is_hex_digit(c.into()));
            assert!(is_octal_digit(c.into()));
        }
        for c in ["8", "9"] {
            assert!(!is_octal_digit(c.into()));
            assert!(is_decimal_digit(c.into()));
            assert!(is_hex_digit(c.into()));
        }
        for c in ["a", "A", "f", "F"] {
            assert!(!is_octal_digit(c.into()));
            assert!(!is_decimal_digit(c.into()));
            assert!(is_hex_digit(c.into()));
        }
        for c in ["g", "三"] {
            assert!(!is_octal_digit(c.into()));
            assert!(!is_decimal_digit(c.into()));
            assert!(!is_hex_digit(c.into()));
        }
    }
}
