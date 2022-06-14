// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

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

#[macro_export]
macro_rules! derive_from {
    (
        $(
            $type:ty {
                $( $func:ident ( $value_type:tt ) ),+ $(,)?
            }
        )+
    ) => {
        $(
            $(
                impl From<$value_type> for $type {
                    fn from(v: $value_type) -> Self {
                        <$type>::$func(v)
                     }
                }
            )+
        )+
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_from_for_struct_new() {
        #[derive(PartialEq, Eq, Debug)]
        struct Data {
            n: i32,
        }
        impl Data {
            pub fn new(n: i32) -> Self {
                Self { n }
            }
        }
        derive_from! {
            Data { new(i32) }
        }
        assert_eq!(Data::from(2), Data::new(2));
    }
    #[test]
    fn test_derive_from_for_enum() {
        #[derive(PartialEq, Eq, Debug)]
        enum Value {
            Int32(i32),
            Int64(i64),
            Bool(bool),
            #[allow(dead_code)]
            OtherBool(bool),
        }
        derive_from! {
            Value {
                Int32(i32),
                Int64(i64),
                Bool(bool),
            }
        }

        assert_eq!(Value::from(10), Value::Int32(10));
        assert_eq!(Value::from(10i64), Value::Int64(10));
        assert_eq!(Value::from(true), Value::Bool(true));
    }

    #[test]
    fn test_derive_from_for_enum_multiple() {
        #[derive(PartialEq, Eq, Debug)]
        enum Num32 {
            Int(i32),
            UInt(u32),
            #[allow(dead_code)]
            OtherInt(i32),
        }
        #[derive(PartialEq, Eq, Debug)]
        enum Num64 {
            Int(i64),
            UInt(u64),
            #[allow(dead_code)]
            OtherInt(i64),
        }

        derive_from! {
            Num32 {
                Int(i32),
                UInt(u32),
            }
            Num64 {
                Int(i64),
                UInt(u64),
            }
        }

        assert_eq!(Num32::from(1), Num32::Int(1));
        assert_eq!(Num32::from(1u32), Num32::UInt(1));
        assert_eq!(Num64::from(1i64), Num64::Int(1));
        assert_eq!(Num64::from(1u64), Num64::UInt(1));
    }
}
