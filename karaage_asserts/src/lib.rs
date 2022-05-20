// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
pub use paste::paste;

#[macro_export]
macro_rules! assert_impl {
    ($type:ident < $trait:ident > ) => {{
        $crate::paste! {
            fn [<assert_impl_ $trait:lower>]<T: ?Sized + $trait>(){}

            [<assert_impl_ $trait:lower>]::<$type>();
        }
    }};
    ($type:ident < $( $trait:ident ),+ $(,)? > ) => {{
        $(
            $crate::assert_impl!($type<$trait>);
        )+
    }};
}

#[macro_export]
macro_rules! fn_test_data_traits {
    ($t:ty) => {
        $crate::paste! {
            #[test]
            fn [<test_ $t:lower _data_traits>]() {
                use std::fmt::Debug;
                $crate::assert_impl!($t<
                    Send, Sync,
                    Clone,
                    Eq, PartialEq,
                    Debug,
                >);
            }
        }
    };
}
#[cfg(test)]
mod tests {
    fn_test_data_traits!(i32);
}
