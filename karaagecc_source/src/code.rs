// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt::{Debug, Display};

use karaagecc_chars::Chars;

#[derive(Clone, Eq, PartialEq, Default)]
pub struct Code {
    s: String,
}

impl From<String> for Code {
    fn from(s: String) -> Self {
        Self { s }
    }
}

impl From<&str> for Code {
    fn from(s: &str) -> Self {
        Self { s: s.to_string() }
    }
}

macro_rules! impl_code_eq {
    ($other: ty) => {
        #[allow(unused_lifetimes)]
        impl<'a> PartialEq<$other> for Code {
            fn eq(&self, other: &$other) -> bool {
                PartialEq::eq(&self.s, other)
            }
        }

        #[allow(unused_lifetimes)]
        impl<'a> PartialEq<Code> for $other {
            fn eq(&self, other: &Code) -> bool {
                PartialEq::eq(self, &other.s)
            }
        }
    };
}

impl_code_eq! {String}
impl_code_eq! {str}
impl_code_eq! {&'a str}

#[test]
fn test_eq() {
    assert_eq!("abc", Code::from("abc"));
    assert_eq!(*"abc", Code::from("abc"));
    assert_eq!("abc".to_string(), Code::from("abc"));
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.s, f)
    }
}

impl Debug for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.s, f)
    }
}

#[test]
fn test_fmt() {
    assert_eq!("abc", format!("{}", Code::from("abc")));
    assert_eq!("\"abc\"", format!("{:?}", Code::from("abc")));
}

impl Code {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn chars(&self) -> Chars<'_> {
        Chars::from(self.s.as_str())
    }

    pub fn is_empty(&self) -> bool {
        self.s.is_empty()
    }

    pub fn len(&self) -> usize {
        self.s.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use karaage_asserts::{assert_eq, *};

    fn_test_data_traits!(Code);

    #[test]
    fn test_new() {
        assert_eq!(Code::new(), Code::default());
    }

    #[test]
    fn test_len() {
        assert!(Code::new().is_empty());
        assert_eq!(Code::new().len(), 0);
        assert!(!Code::from("abc").is_empty());
        assert_eq!(Code::from("abc").len(), 3);
    }

    #[test]
    fn test_chars() {
        use karaagecc_chars::Char;
        let code = Code::from("abc");
        assert_iter_eq!(
            code.chars(),
            [Char::from("a"), Char::from("b"), Char::from("c")],
        );
    }
}
