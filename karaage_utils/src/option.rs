// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::rc::Rc;

pub trait IntoOption<T> {
    fn ok(self) -> Option<T>;
}

impl<T> IntoOption<T> for Option<T> {
    fn ok(self) -> Option<T> {
        self
    }
}

impl<T, E> IntoOption<T> for Result<T, E> {
    fn ok(self) -> Option<T> {
        self.ok()
    }
}

pub trait IntoResult<T, E> {
    fn ok(self) -> Result<T, E>;
}

impl<T> IntoResult<T, Option<T>> for Option<T> {
    fn ok(self) -> Result<T, Option<T>> {
        self.ok_or(None)
    }
}

impl<T, E> IntoResult<T, E> for Result<T, E> {
    fn ok(self) -> Result<T, E> {
        self
    }
}

#[macro_export]
macro_rules! must {
    ($expr:expr) => {
        $crate::IntoResult::ok($expr).unwrap_or_else(|e| unreachable!("{:?}", e))
    };
    ($expr:expr , $msg:expr) => {
        $crate::IntoOption::ok($expr).unwrap_or_else(|| unreachable!($msg))
    };
}

#[cfg(test)]
mod must_tests {
    use crate::*;
    use karaage_asserts::assert_eq;

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
    #[should_panic(expected = "None")]
    fn test_must_none() {
        must!(None)
    }

    #[test]
    #[should_panic(expected = "unexpected none")]
    fn test_must_none_with_message() {
        must!(None, "unexpected none");
    }

    #[test]
    #[should_panic(expected = "10")]
    fn test_must_err() {
        must!(err(10))
    }

    #[test]
    #[should_panic(expected = "unexpected err")]
    fn test_must_err_with_message() {
        must!(err(10), "unexpected err");
    }
}

pub fn clone_option_rc<T>(v: Option<&Rc<T>>) -> Option<Rc<T>> {
    v.map(Rc::clone)
}

#[test]
fn test_clone_option_rc() {
    fn f(v: Option<&Rc<i32>>) -> Option<Rc<i32>> {
        clone_option_rc(v)
    }

    let v = Rc::new(10);
    assert_eq!(Rc::strong_count(&v), 1);

    let v1 = f(Some(&v));
    assert_eq!(v1, Some(Rc::new(10)));
    assert_eq!(Rc::strong_count(&v), 2);

    let v2 = f(None);
    assert_eq!(v2, None);
}
