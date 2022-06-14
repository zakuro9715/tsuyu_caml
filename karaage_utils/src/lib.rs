// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub use derive_more as derives;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum OptionOrResult<T, E> {
    Option(Option<T>),
    Result(Result<T, E>),
}

impl<T, E> OptionOrResult<T, E> {
    pub fn ok(self) -> Option<T> {
        match self {
            Self::Option(o) => o,
            Self::Result(r) => r.ok(),
        }
    }
}

impl<T> From<Option<T>> for OptionOrResult<T, ()> {
    fn from(o: Option<T>) -> Self {
        Self::Option(o)
    }
}

impl<T, E> From<Result<T, E>> for OptionOrResult<T, E> {
    fn from(r: Result<T, E>) -> Self {
        Self::Result(r)
    }
}

#[macro_export]
macro_rules! must {
    ($expr:expr $(, $msg:expr)?) => {
        $crate::OptionOrResult::from($expr).ok().unwrap_or_else(|| unreachable!($($msg)?))
    }
}

#[cfg(test)]
mod must_tests {
    use super::*;
    fn ok<T>(v: T) -> Result<T, ()> {
        Ok(v)
    }

    fn err<E>(v: E) -> Result<(), E> {
        Err(v)
    }

    #[test]
    fn test_must() {
        assert_eq!(must!(Some(10)), 10);
        assert_eq!(must!(ok(10)), 10);
    }

    #[test]
    #[should_panic]
    fn test_must_none() {
        must!(None)
    }

    #[test]
    #[should_panic(expected = "unexpected none")]
    fn test_must_none_with_message() {
        must!(None, "unexpected none");
    }

    #[test]
    #[should_panic]
    fn test_must_err() {
        must!(err(10))
    }

    #[test]
    #[should_panic(expected = "unexpected err")]
    fn test_must_err_with_message() {
        must!(err(10), "unexpected err");
    }
}
