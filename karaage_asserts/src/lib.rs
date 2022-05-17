// Copyright (c) 2022 zakuro <z@kuro.red>. All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
pub use paste::paste;
use std::fmt::Debug;

pub fn assert_send<T: Send>() {}
pub fn assert_sync<T: Sync>() {}
pub fn assert_send_sync<T: Send + Sync>() {}
pub fn assert_data_traits<T: Send + Sync + Clone + Eq + PartialEq + Debug>() {}

#[macro_export]
macro_rules! fn_test_send {
    ($t:ty) => {
        $crate::paste! {
            #[test]
            fn [<test_ $t:lower _send>]() {
                $crate::assert_send::<$t>();
            }
        }
    };
}

#[macro_export]
macro_rules! fn_test_sync {
    ($t:ty) => {
        $crate::paste! {
            #[test]
            fn [<test_ $t:lower _sync>]() {
                $crate::assert_sync::<$t>();
            }
        }
    };
}

#[macro_export]
macro_rules! fn_test_send_sync {
    ($t:ty) => {
        $crate::paste! {
            #[test]
            fn [<test_ $t:lower _send_sync>]() {
                $crate::assert_send_sync::<$t>();
            }
        }
    };
}

#[macro_export]
macro_rules! fn_test_data_traits {
    ($t:ty) => {
        $crate::paste! {
            #[test]
            fn [<test_ $t:lower _data_traits>]() {
                $crate::assert_data_traits::<$t>();
            }
        }
    };
}
#[cfg(test)]
mod tests {
    fn_test_send!(i32);
    fn_test_sync!(i32);
    fn_test_send_sync!(i32);
}
