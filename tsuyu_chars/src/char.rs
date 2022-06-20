// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{fmt, hash::Hash};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct Char<'a>(&'a str);

impl<'a> Char<'a> {
    fn byte(&self) -> u8 {
        self.0.bytes().next().expect("Char is broken")
    }

    pub fn char(&self) -> char {
        self.0.chars().next().expect("Char is broken")
    }

    pub fn ascii(&self) -> Option<u8> {
        let b = self.byte();
        if b.is_ascii() {
            Some(b)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

// --- traits ---

impl<'a> From<&'a str> for Char<'a> {
    fn from(s: &'a str) -> Self {
        Self(s)
    }
}

#[test]
fn test_from_str() {
    let c = '🐈';
    assert_eq!(Char::from(c.to_string().as_str()).char(), c)
}

macro_rules! impl_code_eq {
    ($other: ty) => {
        #[allow(unused_lifetimes)]
        impl<'a> PartialEq<$other> for Char<'_> {
            fn eq(&self, other: &$other) -> bool {
                PartialEq::eq(&self.0, other)
            }
        }

        #[allow(unused_lifetimes)]
        impl<'a> PartialEq<Char<'_>> for $other {
            fn eq(&self, other: &Char<'_>) -> bool {
                PartialEq::eq(self, &other.0)
            }
        }
    };
}

impl PartialEq<str> for Char<'_> {
    fn eq(&self, other: &str) -> bool {
        PartialEq::eq(self.0, other)
    }
}

impl PartialEq<Char<'_>> for str {
    fn eq(&self, other: &Char<'_>) -> bool {
        PartialEq::eq(self, other.0)
    }
}

impl_code_eq! {String}
impl_code_eq! {&'a str}

impl<'a> fmt::Display for Char<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> fmt::Debug for Char<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

// --- traits end --

pub fn is_whitespace(c: Char<'_>) -> bool {
    c.char().is_whitespace()
}

pub fn is_newline(c: Char<'_>) -> bool {
    matches!(c.char(), '\n' | '\r')
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
    use tsuyu_asserts::{assert_eq, assert_ne};
    use rstest::*;

    #[test]
    fn test_char_eq_ne() {
        assert_eq!("🐈", Char::from("🐈"));
        assert_ne!("🐈", Char::from("🐶"));
        assert_eq!(*"🐈", Char::from("🐈"));
        assert_ne!(*"🐈", Char::from("🐶"));
        assert_eq!("🐈".to_string(), Char::from("🐈"));
        assert_ne!("🐈".to_string(), Char::from("🐶"));
    }

    #[test]
    fn test_fmt() {
        let s = "🐈";
        let c = Char::from(s);
        assert_eq!(format!("{}", c), format!("{}", s));
        assert_eq!(format!("{:?}", c), format!("{:?}", s));
    }

    #[test]
    fn test_char() {
        assert_eq!(Char::from("a").char(), 'a');
        assert_eq!(Char::from("🐈").char(), '🐈');
    }

    #[test]
    fn test_len() {
        assert_eq!(Char::from("a").len(), 'a'.len_utf8());
        assert_eq!(Char::from("🐈").len(), '🐈'.len_utf8());
    }

    #[test]
    fn test_is_empty() {
        assert!(!Char::from("a").is_empty());
        assert!(!Char::from("🐈").is_empty());
    }

    #[rstest]
    fn test_whitespaces(#[values(" ", "\t", "\n", "\r")] c: &str) {
        assert!(is_whitespace(c.into()));
    }

    #[rstest]
    fn test_not_whitespaces(#[values("a", "あ")] c: &str) {
        assert!(!is_whitespace(c.into()));
    }

    #[rstest]
    fn test_newline(#[values("\n", "\r")] c: &str) {
        assert!(is_newline(c.into()));
        assert!(is_whitespace(c.into()));
    }

    #[rstest]
    fn test_not_newline(#[values(" ", "\t")] c: &str) {
        assert!(!is_newline(c.into()));
    }

    #[rstest]
    fn test_octal_digits(#[values("0", "3", "7")] c: &str) {
        assert!(is_decimal_digit(c.into()));
        assert!(is_hex_digit(c.into()));
        assert!(is_octal_digit(c.into()));
    }

    #[rstest]
    fn test_decimal_digits(#[values("8", "9")] c: &str) {
        assert!(!is_octal_digit(c.into()));
        assert!(is_decimal_digit(c.into()));
        assert!(is_hex_digit(c.into()));
    }

    #[rstest]
    fn test_hex_digits(#[values("a", "A", "f", "F")] c: &str) {
        assert!(!is_octal_digit(c.into()));
        assert!(!is_decimal_digit(c.into()));
        assert!(is_hex_digit(c.into()));
    }

    #[rstest]
    fn test_not_digits(#[values("g", "三")] c: &str) {
        assert!(!is_octal_digit(c.into()));
        assert!(!is_decimal_digit(c.into()));
        assert!(!is_hex_digit(c.into()));
    }
}
