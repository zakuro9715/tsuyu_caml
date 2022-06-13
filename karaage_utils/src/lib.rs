// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[macro_export]
macro_rules! impl_from_for_enum {
    (
        $(
            $enum:ty {
                $( $variant:ident ( $t:tt ) ),+ $(,)?
            }
        )+
    ) => {
        $(
            $(
                impl From<$t> for $enum {
                    fn from(v: $t) -> Self {
                        <$enum>::$variant(v)
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
    fn test_impl_from_for_enum() {
        #[derive(PartialEq, Eq, Debug)]
        enum Value {
            Int32(i32),
            Int64(i64),
            Bool(bool),
            #[allow(dead_code)]
            OtherBool(bool),
        }
        impl_from_for_enum! {
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
    fn test_impl_from_for_enum_multiple() {
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

        impl_from_for_enum! {
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
